use crate::common::error::AppError;
use crate::domain::models::Customer;

pub trait CustomerRepoTrait: Send + Sync {
    fn list(&self) -> Result<Vec<Customer>, AppError>;
    fn get_by_mdoc(&self, mdoc: i32) -> Result<Option<Customer>, AppError>;
    fn update_updated_date(
        &self,
        mdoc: i32,
        new_updated: chrono::NaiveDateTime,
    ) -> Result<(), AppError>;
}
