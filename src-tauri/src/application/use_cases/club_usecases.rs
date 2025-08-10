use crate::common::error::AppError;
use crate::domain::models::{ClubImport, ClubTransaction, Customer};
use crate::domain::repos::{ClubImportRepoTrait, ClubTransactionRepoTrait, CustomerRepoTrait};
use crate::try_log;
use std::sync::Arc;

pub struct ClubUseCases {
    customer_repo: Arc<dyn CustomerRepoTrait>,
    tx_repo: Arc<dyn ClubTransactionRepoTrait>,
    import_repo: Arc<dyn ClubImportRepoTrait>,
}

impl ClubUseCases {
    pub fn new(
        customer_repo: Arc<dyn CustomerRepoTrait>,
        tx_repo: Arc<dyn ClubTransactionRepoTrait>,
        import_repo: Arc<dyn ClubImportRepoTrait>,
    ) -> Self {
        Self {
            customer_repo,
            tx_repo,
            import_repo,
        }
    }

    pub fn search_customers(
        &self,
        page: i32,
        search: Option<String>,
    ) -> Result<Vec<(Customer, i32)>, AppError> {
        let limit = 10;
        let offset = page.saturating_sub(1) * limit;
        let res = try_log!(
            self.customer_repo.search(limit, offset, search),
            "ClubUseCases::search_customers"
        );
        Ok(res)
    }

    pub fn count_customers(&self, search: Option<String>) -> Result<i32, AppError> {
        let count = try_log!(
            self.customer_repo.count(search),
            "ClubUseCases::count_customers"
        );
        Ok(count)
    }

    pub fn search_club_transactions(
        &self,
        page: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(ClubTransaction, Option<String>)>, AppError> {
        let limit = 10;
        let offset = page.saturating_sub(1) * limit;
        let res = try_log!(
            self.tx_repo.search(limit, offset, date, search),
            "ClubUseCases::search_club_transactions"
        );
        Ok(res)
    }

    pub fn count_club_transactions(
        &self,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<i32, AppError> {
        let count = try_log!(
            self.tx_repo.count(date, search),
            "ClubUseCases::count_club_transactions"
        );
        Ok(count)
    }

    pub fn list_club_imports(&self) -> Result<Vec<ClubImport>, AppError> {
        let res = try_log!(self.import_repo.list(), "ClubUseCases::list_club_imports");
        Ok(res)
    }
}
