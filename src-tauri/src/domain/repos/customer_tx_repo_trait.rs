use crate::common::error::AppError;
use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::domain::models::CustomerTransaction;

pub type SaleDetailsTuple = (CustomerTransaction, Vec<(CustomerTxDetail, String)>, i32);

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

    fn get_with_details_and_balance(&self, _order_id: i32) -> Result<SaleDetailsTuple, AppError>;
}
