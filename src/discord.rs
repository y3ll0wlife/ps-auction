use crate::models::database::AuctionItem;
use crate::models::discord::ItemChanges;
use crate::models::ps_auction_container::Item;
use serenity::prelude::SerenityError;
use serenity::{http::Http, model::channel::Embed, model::webhook::Webhook, utils::Colour};
use std::env;

pub async fn new_item(item: &Item) -> Result<(), SerenityError> {
    let http: Http = Http::new("token");
    let token = env::var("WEBHOOK_URL").expect("missing WEBHOOK_URL in .env");
    let webhook = Webhook::from_url(&http, &token).await?;

    let embed = Embed::fake(|e| {
        e.colour(Colour::from_rgb(255, 169, 249))
            .title(item.name.clone())
            .field(
                "Price",
                &item
                    .leading_bid
                    .clone()
                    .unwrap_or_else(|| String::from("0.00")),
                true,
            )
            .field("Next Bid", &item.next_bid, true)
            .field("Slug", &item.slug, true)
            .field("Market Value", &item.market_value, true)
            .field(
                "Current Conversion",
                &item
                    .current_conversion
                    .clone()
                    .unwrap_or_else(|| String::from("")),
                true,
            )
            .field("Location", &item.location, true)
            .field("AI Cancelled", &item.ai_cancelled, true)
            .field(
                "Link to product",
                format!("[Link]({})", &item.get_item_url()),
                true,
            )
            .field("End in", &item.get_end_time(), true)
            .image(&item.thumbnail)
    });

    match webhook
        .execute(&http, true, |w| {
            w.username("PS Auction").embeds(vec![embed])
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn send_update(
    item: &Item,
    previous_item: &AuctionItem,
    changes: &Vec<ItemChanges>,
) -> Result<(), SerenityError> {
    let http = Http::new("token");
    let token = env::var("WEBHOOK_URL").expect("missing WEBHOOK_URL in .env");
    let webhook = Webhook::from_url(&http, &token).await?;

    let mut description: Vec<String> = vec![];
    for change in changes {
        match change {
            ItemChanges::Price => description.push(format!(
                "**Price**\n{} ➜ {}",
                format!("{}.00", previous_item.item_price),
                item.leading_bid
                    .clone()
                    .unwrap_or_else(|| String::from("0.00"))
            )),
        }
    }

    let embed = Embed::fake(|e| {
        e.colour(Colour::from_rgb(255, 169, 249))
            .title(item.name.clone())
            .description(description.join("\n\n"))
            .field(
                "Link to product",
                format!("[Link]({})", &item.get_item_url()),
                true,
            )
            .field("End in", &item.get_end_time(), true)
            .image(&item.thumbnail)
    });

    match webhook
        .execute(&http, true, |w| {
            w.username("PS Auction").embeds(vec![embed])
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
