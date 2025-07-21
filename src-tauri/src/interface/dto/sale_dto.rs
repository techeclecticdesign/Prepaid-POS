use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct SaleItemDto {
    pub upc: String,
    pub desc: String,
    pub quantity: i32,
    pub price: i32,
}

#[derive(Deserialize, Clone)]
pub struct SaleDto {
    pub customer_mdoc: i32,
    pub operator_mdoc: i32,
    pub operator_name: String,
    pub customer_name: String,
    pub items: Vec<SaleItemDto>,
}
