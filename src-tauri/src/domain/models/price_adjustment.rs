use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PriceAdjustment {
    pub id: i32, // auto‑assigned primary key
    pub operator_mdoc: i32,
    pub upc: i64,
    pub old: i32,
    pub new: i32,
    pub created_at: Option<NaiveDateTime>,
}
