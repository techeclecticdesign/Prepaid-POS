use crate::common::error::AppError;
use crate::domain::models::InventoryTransaction;

pub trait InventoryTransactionRepoTrait: Send + Sync {
    fn get_by_id(&self, id: i32) -> Result<Option<InventoryTransaction>, AppError>;
    fn create(&self, tx: &InventoryTransaction) -> Result<(), AppError>;
    fn list_for_product(&self, upc: String) -> Result<Vec<InventoryTransaction>, AppError>;
    fn list_for_operator(&self, operator_mdoc: i32) -> Result<Vec<InventoryTransaction>, AppError>;
    fn list_for_customer(&self, customer_mdoc: i32) -> Result<Vec<InventoryTransaction>, AppError>;
    fn list_for_today(&self) -> Result<Vec<InventoryTransaction>, AppError>;
    fn list(&self) -> Result<Vec<InventoryTransaction>, AppError>;
    fn search(
        &self,
        limit: i32,
        offset: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(InventoryTransaction, String, String)>, AppError>;
    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i32, AppError>;
    fn create_with_tx(
        &self,
        a: &InventoryTransaction,
        tx: &rusqlite::Transaction<'_>,
    ) -> Result<(), AppError>;
}
