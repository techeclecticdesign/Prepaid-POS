use crate::domain::models::CustomerTransaction;

pub struct SalesReportDetails {
    pub tx: CustomerTransaction,
    pub customer_name: String,
    pub item_count: i32,
    pub order_total: i32,
    pub details: Vec<SalesReportDetailRow>,
}

pub struct SalesReportDetailRow {
    pub detail_id: i32,
    pub order_id: i32,
    pub upc: String,
    pub quantity: i32,
    pub price: i32,
    pub product_name: String,
}
