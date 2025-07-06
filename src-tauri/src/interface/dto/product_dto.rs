use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProductDto {
    pub upc: i64,
    pub desc: String,
    pub category: String,
    pub price: i32,              // integer cents
    pub updated: String,         // RFC 3339 timestamp
    pub added: String,           // RFC 3339 timestamp
    pub deleted: Option<String>, // optional RFC 3339 timestamp
}

#[derive(Serialize, Deserialize)]
pub struct RemoveProductDto {
    pub upc: i64,
}
