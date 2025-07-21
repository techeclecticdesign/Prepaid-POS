use crate::domain::models::CustomerTransaction;
use serde::Serialize;

#[derive(Serialize)]
pub struct PrinterDto {
    pub name: String,
}

#[derive(Serialize)]
pub struct PrintableLineItem {
    pub upc: String,
    pub desc: String,
    pub quantity: i32,
    pub price: i32,
}

#[derive(Serialize)]
pub struct PrintableSaleDto {
    pub transaction: CustomerTransaction,
    pub items: Vec<PrintableLineItem>,
    pub balance: i32,
}
