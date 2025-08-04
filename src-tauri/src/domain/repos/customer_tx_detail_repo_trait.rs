use crate::common::error::AppError;
use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::domain::report_models::product_sales::{ProductSalesByCategory, SalesTotals};
use chrono::NaiveDateTime;

pub trait CustomerTxDetailRepoTrait: Send + Sync {
    // insert a new detail; detail.detail_id==0 will auto‐assign
    fn create(&self, detail: &CustomerTxDetail) -> Result<(), AppError>;

    // list all details for a given order_id
    fn list_by_order(&self, order_id: i32) -> Result<Vec<(CustomerTxDetail, String)>, AppError>;
    fn create_with_tx(
        &self,
        d: &CustomerTxDetail,
        tx: &rusqlite::Transaction<'_>,
    ) -> Result<i32, AppError>;
    fn sales_by_category(
        &self,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<ProductSalesByCategory>, AppError>;
    fn get_sales_totals(
        &self,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<SalesTotals, AppError>;
}
