pub struct ProductInventoryReport {
    pub category: String,
    pub upc: Option<String>,  // None for summary rows
    pub name: Option<String>, // None for summary rows
    pub price: Option<i32>,   // None for summary rows
    pub quantity: i32,
    pub total: i32,       // price * quantity
    pub is_summary: bool, // true for the summed lines once per category
}

pub struct ProductInventoryTotals {
    pub total_quantity: i32,
    pub total_value: i32,
}
