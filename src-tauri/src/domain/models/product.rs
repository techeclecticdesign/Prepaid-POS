use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub upc: i64,
    pub desc: String,
    pub category: String,
    pub price: i32, // price stored as integer cents
    pub updated: NaiveDateTime,
    pub added: NaiveDateTime,
    pub deleted: Option<NaiveDateTime>,
}
