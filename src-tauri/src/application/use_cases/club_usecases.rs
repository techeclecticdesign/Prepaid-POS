use crate::common::error::AppError;
use crate::domain::models::{ClubImport, ClubTransaction, Customer};
use crate::domain::repos::{ClubImportRepoTrait, ClubTransactionRepoTrait, CustomerRepoTrait};
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
        self.customer_repo.search(limit, offset, search)
    }

    pub fn count_customers(&self, search: Option<String>) -> Result<i32, AppError> {
        self.customer_repo.count(search)
    }

    pub fn search_club_transactions(
        &self,
        page: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(ClubTransaction, Option<String>)>, AppError> {
        let limit = 10;
        let offset = page.saturating_sub(1) * limit;
        self.tx_repo.search(limit, offset, date, search)
    }

    pub fn count_club_transactions(
        &self,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<i32, AppError> {
        self.tx_repo.count(date, search)
    }

    pub fn list_club_imports(&self) -> Result<Vec<ClubImport>, AppError> {
        self.import_repo.list()
    }
}
