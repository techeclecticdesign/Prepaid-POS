use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub upc: String,
    pub desc: String,
    pub category: String,
    pub price: i32, // price stored as integer cents
    pub updated: Option<NaiveDateTime>,
    pub added: Option<NaiveDateTime>,
    pub deleted: Option<NaiveDateTime>,
}
