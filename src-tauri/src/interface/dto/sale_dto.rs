use serde::Deserialize;

#[derive(Deserialize)]
pub struct SaleItemDto {
    pub upc: String,
    pub quantity: i32,
    pub price: i32,
}

#[derive(Deserialize)]
pub struct SaleDto {
    pub customer_mdoc: i32,
    pub operator_mdoc: i32,
    pub items: Vec<SaleItemDto>,
}
