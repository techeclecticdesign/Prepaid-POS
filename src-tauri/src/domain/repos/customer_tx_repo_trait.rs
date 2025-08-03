use crate::common::error::AppError;
use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::domain::models::CustomerTransaction;
use crate::domain::report_models::sales_details::SalesReportDetails;
use chrono::NaiveDateTime;

pub type SaleDetailsTuple = (CustomerTransaction, Vec<(CustomerTxDetail, String)>, i32);

pub trait CustomerTransactionRepoTrait: Send + Sync {
    fn create(&self, tx: &CustomerTransaction) -> Result<(), AppError>;
    fn get(&self, order_id: i32) -> Result<Option<CustomerTransaction>, AppError>;
    fn list(&self) -> Result<Vec<CustomerTransaction>, AppError>;
    fn search(
        &self,
        limit: i32,
        offset: i32,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(CustomerTransaction, String, i32)>, AppError>;

    fn count(
        &self,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<i32, AppError>;

    fn create_with_tx(
        &self,
        tx_data: &CustomerTransaction,
        tx: &rusqlite::Transaction<'_>,
    ) -> Result<i32, AppError>;

    fn get_with_details_and_balance(&self, _order_id: i32) -> Result<SaleDetailsTuple, AppError>;

    // Get total for mdoc between `week_start` (inclusive) and `week_start + 7 days` (exclusive).
    fn get_weekly_spent(
        &self,
        customer_mdoc: i32,
        week_start: NaiveDateTime,
    ) -> Result<i32, AppError>;

    fn get_sales_details_data(
        &self,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<SalesReportDetails>, AppError>;
}
