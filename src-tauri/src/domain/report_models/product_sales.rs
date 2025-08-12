pub struct ProductSalesByCategory {
    pub category: String,
    pub upc: String,
    pub name: String,
    pub quantity_sold: i32,
    pub price: i32,
    pub total_sales: i32,
    pub is_summary: bool,
}

#[derive(Clone, Copy)]
pub struct SalesTotals {
    pub total_quantity: i32,
    pub total_value: i32,
}
