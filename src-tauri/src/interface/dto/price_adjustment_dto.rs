use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PriceAdjustmentDto {
    pub id: i32,
    pub upc: i64,
    pub old: i32,
    pub new: i32,
    pub operator_mdoc: i32,
    pub created_at: String, // RFC3339
}
