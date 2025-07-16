use crate::common::error::AppError;
use crate::domain::models::ClubImport;

pub trait ClubImportRepoTrait: Send + Sync {
    fn list(&self) -> Result<Vec<ClubImport>, AppError>;
    fn get_by_id(&self, id: i32) -> Result<Option<ClubImport>, AppError>;
    fn create(&self, import: &ClubImport) -> Result<(), AppError>;
    fn search(
        &self,
        limit: i64,
        offset: i64,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<ClubImport>, AppError>;

    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i64, AppError>;
}
