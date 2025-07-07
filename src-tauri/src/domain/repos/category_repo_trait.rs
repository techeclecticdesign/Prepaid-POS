use crate::common::error::AppError;
use crate::domain::models::category::Category;

pub trait CategoryRepoTrait: Send + Sync {
    fn list(&self) -> Result<Vec<Category>, AppError>;
    fn list_active(&self) -> Result<Vec<Category>, AppError>;
    fn get_by_id(&self, id: i64) -> Result<Option<Category>, AppError>;
    fn create(&self, c: String) -> Result<(), AppError>;
    fn soft_delete(&self, id: i64) -> Result<(), AppError>;
    fn get_by_name(&self, name: &str) -> Result<Option<Category>, AppError>;
    fn undelete(&self, id: i64) -> Result<(), AppError>;
}
