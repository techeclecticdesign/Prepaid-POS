use crate::common::error::AppError;
use crate::domain::models::Operator;

pub trait OperatorRepoTrait: Send + Sync {
    fn get_by_mdoc(&self, mdoc: i32) -> Result<Option<Operator>, AppError>;
    fn create(&self, operator: &Operator) -> Result<(), AppError>;
    fn update_by_mdoc(&self, operator: &Operator) -> Result<(), AppError>;
    fn list(&self) -> Result<Vec<Operator>, AppError>;
}
