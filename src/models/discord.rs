use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemChanges {
    Price,
    Active,
    Cancelled,
    Visible,
}
