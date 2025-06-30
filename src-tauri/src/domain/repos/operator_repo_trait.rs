use crate::domain::models::Operator;
use crate::error::AppError;

pub trait OperatorRepoTrait: Send + Sync {
    fn get_by_id(&self, id: i32) -> Result<Option<Operator>, AppError>;
    fn create(&self, operator: &Operator) -> Result<(), AppError>;
    fn update_by_id(&self, operator: &Operator) -> Result<(), AppError>;
    fn list(&self) -> Result<Vec<Operator>, AppError>;
}
