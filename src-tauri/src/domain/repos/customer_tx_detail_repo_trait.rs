use crate::common::error::AppError;
use crate::domain::models::customer_tx_detail::CustomerTxDetail;

pub trait CustomerTxDetailRepoTrait: Send + Sync {
    // insert a new detail; detail.detail_id==0 will auto‐assign
    fn create(&self, detail: &CustomerTxDetail) -> Result<(), AppError>;

    // list all details for a given order_id
    fn list_by_order(&self, order_id: i32) -> Result<Vec<(CustomerTxDetail, String)>, AppError>;
}
