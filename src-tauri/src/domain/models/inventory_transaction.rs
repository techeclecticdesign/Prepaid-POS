use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InventoryTransaction {
    pub id: i32, // auto‑assigned PK
    pub upc: i64,
    pub quantity_change: i32,
    pub operator_mdoc: i32,
    pub customer_mdoc: Option<i32>,
    pub ref_order_id: Option<i32>,
    pub reference: Option<String>,
    pub created_at: NaiveDateTime,
}
