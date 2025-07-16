use crate::common::error::AppError;
use crate::domain::models::InventoryTransaction;

pub trait InventoryTransactionRepoTrait: Send + Sync {
    fn get_by_id(&self, id: i64) -> Result<Option<InventoryTransaction>, AppError>;
    fn create(&self, tx: &InventoryTransaction) -> Result<(), AppError>;
    fn list_for_product(&self, upc: String) -> Result<Vec<InventoryTransaction>, AppError>;
    fn list_for_operator(&self, operator_mdoc: i32) -> Result<Vec<InventoryTransaction>, AppError>;
    fn list_for_customer(&self, customer_mdoc: i32) -> Result<Vec<InventoryTransaction>, AppError>;
    fn list_for_today(&self) -> Result<Vec<InventoryTransaction>, AppError>;
    fn list(&self) -> Result<Vec<InventoryTransaction>, AppError>;
    fn search(
        &self,
        limit: i64,
        offset: i64,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<InventoryTransaction>, AppError>;
    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i64, AppError>;
}
