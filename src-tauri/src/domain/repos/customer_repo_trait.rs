use crate::common::error::AppError;
use crate::domain::models::Customer;

pub trait CustomerRepoTrait: Send + Sync {
    fn list(&self) -> Result<Vec<Customer>, AppError>;
    fn get_by_mdoc(&self, mdoc: i32) -> Result<Option<Customer>, AppError>;
    fn update(&self, customer: &Customer) -> Result<(), AppError>;
    fn create(&self, customer: &Customer) -> Result<(), AppError>;
    fn search(
        &self,
        limit: i64,
        offset: i64,
        search: Option<String>,
    ) -> Result<Vec<Customer>, AppError>;

    fn count(&self, search: Option<String>) -> Result<i64, AppError>;
}
