use crate::models::ps_auction_container::{Item, PsAuctionContainer};
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct PsAuction {
    pub items: Vec<Item>,
}

impl PsAuction {
    pub async fn scrape(base_url: String) -> PsAuction {
        let mut auction = PsAuction { items: vec![] };
        let mut url = base_url.clone();

        loop {
            let response = reqwest::get(&url).await;
            let html_content = response.unwrap().text().await.unwrap();
            let fragment = Html::parse_fragment(&html_content);

            let selector = Selector::parse("div.auctions-list").unwrap();

            let mut item_container = String::new();
            for element in fragment.select(&selector) {
                if let Some(src) = element.value().attr("psitemsearchcontainer") {
                    item_container = src.to_string();
                    break;
                }
            }

            let mut container: PsAuctionContainer = serde_json::from_str(&item_container).unwrap();

            println!(
                "Fetched page number {} and found {} items (total items: {})",
                container.current_page,
                container.items.len(),
                auction.items.len()
            );

            auction.items.append(&mut container.items);
            url = format!("{}/sida={}", base_url, container.next_page);

            if !container.has_next_page {
                break;
            }
        }

        auction
    }
}
