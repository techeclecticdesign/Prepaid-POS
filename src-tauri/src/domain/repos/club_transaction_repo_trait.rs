use crate::common::error::AppError;
use crate::domain::models::ClubTransaction;

pub trait ClubTransactionRepoTrait: Send + Sync {
    fn list(&self) -> Result<Vec<ClubTransaction>, AppError>;
    fn get_by_id(&self, id: i32) -> Result<Option<ClubTransaction>, AppError>;
    fn create(&self, tx: &ClubTransaction) -> Result<(), AppError>;
}
