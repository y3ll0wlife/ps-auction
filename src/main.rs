mod discord;
mod models;
mod scrape_items;

use dotenv::dotenv;
use models::{
    database::{Auction, AuctionItem},
    discord::ItemChanges,
};
use scrape_items::PsAuction;
use sqlx::{Connection, PgConnection};
use std::env;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut conn =
        PgConnection::connect(&env::var("DATABASE_URL").expect("missing DATABASE_URL in .env"))
            .await?;

    let auctions = sqlx::query_as::<_, Auction>("SELECT * FROM auctions")
        .fetch_all(&mut conn)
        .await?;

    for auction in auctions {
        let scrape = PsAuction::scrape(auction.url).await;

        let auction_items = sqlx::query_as::<_, AuctionItem>(
            format!(
                "SELECT * FROM auctionitems where auction_id = {}",
                auction.auction_id
            )
            .as_str(),
        )
        .fetch_all(&mut conn)
        .await?;

        if auction_items.len() == 0 {
            println!("Fully new auction, adding all items");
            for item in &scrape.items {
                let price = item
                    .leading_bid
                    .clone()
                    .unwrap_or_else(|| String::from("0"))
                    .replace(".00", "")
                    .parse::<i32>()?;

                sqlx::query!(
                    r#"
                    INSERT INTO auctionitems (item_id, auction_id, item_name, item_price, item_url, is_active, is_cancelled, is_visible)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    RETURNING *
                    "#,
                    item.id,
                    auction.auction_id,
                    item.name,
                    price,
                    item.get_item_url(),
                    item.active,
                    item.cancelled,
                    item.visible
                )
                .fetch_all(&mut conn)
                .await?;

                discord::new_item(item).await?;
            }
            break;
        }

        for item in scrape.items {
            let mut changes: Vec<ItemChanges> = vec![];

            let item_price = item
                .leading_bid
                .clone()
                .unwrap_or_else(|| String::from("0"))
                .replace(".00", "")
                .parse::<i32>()?;
            let db_item = match auction_items.iter().find(|x| x.item_id == item.id) {
                Some(i) => i,
                None => {
                    println!("New item was added to the auction");

                    let i = sqlx::query!(
                        r#"
                        INSERT INTO auctionitems (item_id, auction_id, item_name, item_price, item_url, is_active, is_cancelled, is_visible)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                        RETURNING *
                        "#,
                        item.id,
                        auction.auction_id,
                        item.name,
                        item_price,
                        item.get_item_url(),
                        item.active,
                        item.cancelled,
                        item.visible
                    )
                    .fetch_one(&mut conn)
                    .await?;

                    discord::new_item(&item).await?;

                    &AuctionItem {
                        auction_id: i.auction_id,
                        item_id: i.item_id,
                        item_price: i.item_price,
                        item_name: i.item_name,
                        item_url: i.item_url,
                        is_active: i.is_active,
                        is_cancelled: i.is_cancelled,
                        is_visible: i.is_visible,
                    }
                }
            };

            if item_price != db_item.item_price {
                changes.push(ItemChanges::Price);

                sqlx::query!(
                    r#"
                    UPDATE auctionitems
                    SET item_price = $1
                    WHERE item_id = $2
                    "#,
                    item_price,
                    db_item.item_id
                )
                .execute(&mut conn)
                .await?;
            }

            if item.active != db_item.is_active {
                changes.push(ItemChanges::Active);

                sqlx::query!(
                    r#"
                    UPDATE auctionitems
                    SET is_active = $1
                    WHERE item_id = $2
                    "#,
                    item.active,
                    db_item.item_id
                )
                .execute(&mut conn)
                .await?;
            }

            if item.cancelled != db_item.is_cancelled {
                changes.push(ItemChanges::Cancelled);

                sqlx::query!(
                    r#"
                    UPDATE auctionitems
                    SET is_cancelled = $1
                    WHERE item_id = $2
                    "#,
                    item.cancelled,
                    db_item.item_id
                )
                .execute(&mut conn)
                .await?;
            }

            if item.visible != db_item.is_visible {
                changes.push(ItemChanges::Visible);

                sqlx::query!(
                    r#"
                    UPDATE auctionitems
                    SET is_visible = $1
                    WHERE item_id = $2
                    "#,
                    item.visible,
                    db_item.item_id
                )
                .execute(&mut conn)
                .await?;
            }

            if changes.len() > 0 {
                println!(
                    "Following changes where made to item {}: {:#?}",
                    item.name, changes
                );

                discord::send_update(&item, &db_item, &changes).await?;
            }
        }
    }

    Ok(())
}
