use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PsAuctionContainer {
    pub total: String,
    pub pagination: Vec<Pagination>,
    #[serde(rename = "current")]
    pub current_page: i64,
    #[serde(rename = "prev")]
    pub previous_page: i64,
    #[serde(rename = "next")]
    pub next_page: i64,
    #[serde(rename = "hasprev")]
    pub has_previous_page: bool,
    #[serde(rename = "hasnext")]
    pub has_next_page: bool,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub label: i64,
    pub active: bool,
    pub page: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub slug: String,
    pub thumbnail: String,
    #[serde(rename = "thumbnailcontenttype")]
    pub thumbnail_content_type: String,
    pub number: String,
    pub name: String,
    #[serde(rename = "endtime")]
    pub end_time: String,
    pub location: String,
    pub site: String,
    pub transsite: String,
    pub active: bool,
    pub cancelled: bool,
    #[serde(rename = "cancellationmessage")]
    pub cancellation_message: Value,
    pub icons: Vec<Icon>,
    #[serde(rename = "reachedreservationprice")]
    pub reached_reservation_price: bool,
    pub zero_reserve_price: bool,
    #[serde(rename = "leadingbid")]
    pub leading_bid: Option<String>,
    pub leading: bool,
    pub leading_with_shipping: bool,
    pub leading_without_shipping: bool,
    pub vat: i64,
    #[serde(rename = "pf_vat")]
    pub pf_vat: i64,
    #[serde(rename = "marketvalue")]
    pub market_value: String,
    pub orig_market_value: String,
    #[serde(rename = "purchasefee")]
    pub purchase_fee: Option<String>,
    #[serde(rename = "purchasefeepercent")]
    pub purchase_fee_percent: Option<String>,
    pub min_service_fee: Option<i64>,
    pub strategy: String,
    #[serde(rename = "currentconversion")]
    pub current_conversion: Option<String>,
    pub user_is_person: bool,
    pub user_is_admin: bool,
    pub user_leading_sum: String,
    pub currency: String,
    pub visible: bool,
    #[serde(rename = "maxbid")]
    pub max_bid: Value,
    pub is_countdown: i64,
    pub has_recent_bid: i64,
    pub next_bid: String,
    #[serde(rename = "auctionended")]
    pub auction_ended: bool,
    #[serde(rename = "companybiddersonly")]
    pub company_bidders_only: bool,
    #[serde(rename = "aicancelled")]
    pub ai_cancelled: bool,
}

impl Item {
    pub fn get_item_url(&self) -> String {
        format!(
            "https://psauction.se/item/view/{}/{}",
            self.number, self.slug
        )
    }

    pub fn get_end_time(&self) -> String {
        let naive_datetime =
            NaiveDateTime::parse_from_str(&self.end_time, "%Y-%m-%d %H:%M").unwrap();
        let unix = naive_datetime.and_utc().timestamp();

        format!("<t:{}:F> (<t:{}:R>)", unix, unix)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
