use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomerTxDetail {
    pub detail_id: i32, // PK, auto‑assigned if 0
    pub order_id: i32,
    pub upc: String,
    pub quantity: i32,
    pub price: i32,
}
