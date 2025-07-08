#[derive(serde::Serialize, serde::Deserialize)]
pub struct PriceAdjustmentDto {
    pub upc: i64,
    pub old: i32,
    pub new: i32,
    pub operator_mdoc: i32,
    pub created_at: Option<String>, // RFC3339
}
