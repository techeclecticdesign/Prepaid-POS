use crate::common::error::AppError;
use crate::domain::models::CustomerTransaction;

pub trait CustomerTransactionRepoTrait: Send + Sync {
    fn create(&self, tx: &CustomerTransaction) -> Result<(), AppError>;
    fn get(&self, order_id: i32) -> Result<Option<CustomerTransaction>, AppError>;
    fn list(&self) -> Result<Vec<CustomerTransaction>, AppError>;
}
