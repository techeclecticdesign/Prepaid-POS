use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InventoryTransactionDto {
    pub id: i32,
    pub upc: i64,
    pub quantity_change: i32,
    pub reference: String,
    pub operator_mdoc: i32,
    pub customer_mdoc: Option<i32>,
    pub ref_order_id: Option<i32>,
    pub created_at: String, // RFC3339
}
