use crate::common::error::AppError;

pub trait WeeklyLimitRepoTrait: Send + Sync {
    fn get_limit(&self) -> Result<i32, AppError>;
    fn set_limit(&self, limit: i32) -> Result<(), AppError>;
}
