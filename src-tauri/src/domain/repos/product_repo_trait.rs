use crate::common::error::AppError;
use crate::domain::models::Product;
use crate::domain::report_models::product_inventory::ProductInventoryReport;
use crate::domain::report_models::product_inventory::ProductInventoryTotals;

pub trait ProductRepoTrait: Send + Sync {
    fn get_by_upc(&self, upc: String) -> Result<Option<Product>, AppError>;
    fn get_price(&self, upc: String) -> Result<i32, AppError>;
    fn create(&self, product: &Product) -> Result<(), AppError>;
    fn update_by_upc(&self, product: &Product) -> Result<(), AppError>;
    fn update_by_upc_with_tx(
        &self,
        product: &Product,
        tx: &rusqlite::Transaction<'_>,
    ) -> Result<(), AppError>;
    fn list(&self) -> Result<Vec<Product>, AppError>;
    fn search(
        &self,
        desc_like: Option<String>,
        category: Option<String>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<(Product, i32)>, AppError>;
    fn count(&self, desc_like: Option<String>, category: Option<String>) -> Result<i32, AppError>;
    fn report_by_category(&self) -> Result<Vec<ProductInventoryReport>, AppError>;
    fn get_inventory_totals(&self) -> Result<ProductInventoryTotals, AppError>;
}
