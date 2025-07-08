use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ProductDto {
    pub upc: i64,
    pub desc: String,
    pub category: String,
    pub price: i32,              // integer cents
    pub updated: String,         // RFC 3339 timestamp
    pub added: String,           // RFC 3339 timestamp
    pub deleted: Option<String>, // optional RFC 3339 timestamp
}

#[derive(Serialize)]
pub struct ProductSearchResult {
    pub products: Vec<ProductDto>,
    pub total_count: u32,
}

#[derive(Deserialize)]
pub struct CreateProductDto {
    pub upc: i64,
    pub desc: String,
    pub category: String,
    pub price: i32,
}
