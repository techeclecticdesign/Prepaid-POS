use crate::common::error::AppError;
use crate::domain::models::Product;

pub trait ProductRepoTrait: Send + Sync {
    fn get_by_upc(&self, upc: i64) -> Result<Option<Product>, AppError>;
    fn get_price(&self, upc: i64) -> Result<i32, AppError>;
    fn create(&self, product: &Product) -> Result<(), AppError>;
    fn update_by_upc(&self, product: &Product) -> Result<(), AppError>;
    fn update_by_upc_with_tx(
        &self,
        product: &Product,
        tx: &rusqlite::Transaction<'_>,
    ) -> Result<(), AppError>;
    fn list(&self) -> Result<Vec<Product>, AppError>;
}
