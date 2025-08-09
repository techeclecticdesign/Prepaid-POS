use crate::common::error::AppError;
use crate::domain::models::ClubTransaction;
use crate::domain::report_models::club_import_report::{ClubTransactionWithTotal, PeriodTotals};
use chrono::NaiveDateTime;

pub trait ClubTransactionRepoTrait: Send + Sync {
    fn list(&self) -> Result<Vec<ClubTransaction>, AppError>;
    fn get_by_import_id_with_total(
        &self,
        import_id: i32,
        start_date: Option<NaiveDateTime>,
    ) -> Result<Vec<ClubTransactionWithTotal>, AppError>;
    fn create(&self, tx: &ClubTransaction) -> Result<(), AppError>;
    fn search(
        &self,
        limit: i32,
        offset: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(ClubTransaction, Option<String>)>, AppError>;
    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i32, AppError>;
    fn get_account_total(&self) -> Result<i32, AppError>;
    fn get_period_sums_for_import(&self, import_id: i32) -> Result<PeriodTotals, AppError>;
}
