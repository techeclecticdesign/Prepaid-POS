use crate::common::error::AppError;
use crate::domain::models::PriceAdjustment;

pub trait PriceAdjustmentRepoTrait: Send + Sync {
    fn create(&self, adj: &PriceAdjustment) -> Result<(), AppError>;
    fn create_with_tx(
        &self,
        adj: &PriceAdjustment,
        tx: &rusqlite::Transaction<'_>,
    ) -> Result<(), AppError>;
    fn get_by_id(&self, id: i64) -> Result<Option<PriceAdjustment>, AppError>;
    fn list_for_product(&self, upc: String) -> Result<Vec<PriceAdjustment>, AppError>;
    fn list_for_operator(&self, operator_mdoc: i32) -> Result<Vec<PriceAdjustment>, AppError>;
    fn list_for_today(&self) -> Result<Vec<PriceAdjustment>, AppError>;
    fn list(&self) -> Result<Vec<PriceAdjustment>, AppError>;
    fn search(
        &self,
        limit: i64,
        offset: i64,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<PriceAdjustment>, AppError>;
    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i64, AppError>;
}
