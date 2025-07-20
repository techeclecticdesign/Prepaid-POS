use crate::common::error::AppError;
use crate::domain::models::CustomerTransaction;

pub trait CustomerTransactionRepoTrait: Send + Sync {
    fn create(&self, tx: &CustomerTransaction) -> Result<(), AppError>;
    fn get(&self, order_id: i32) -> Result<Option<CustomerTransaction>, AppError>;
    fn list(&self) -> Result<Vec<CustomerTransaction>, AppError>;
    fn search(
        &self,
        limit: i64,
        offset: i64,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(CustomerTransaction, String, i64)>, AppError>;

    fn count(
        &self,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<i64, AppError>;

    fn create_with_tx(
        &self,
        tx_data: &CustomerTransaction,
        tx: &rusqlite::Transaction<'_>,
    ) -> Result<(), AppError>;
}
