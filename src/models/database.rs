use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Auction {
    pub auction_id: i32,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuctionItem {
    pub item_id: i32,
    pub auction_id: i32,
    pub item_name: String,
    pub item_price: i32,
    pub item_url: String,
    pub is_active: bool,
    pub is_cancelled: bool,
    pub is_visible: bool,
}
