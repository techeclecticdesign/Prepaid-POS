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

    pub fn list_customers(&self) -> Result<Vec<Customer>, AppError> {
        self.customer_repo.list()
    }

    pub fn get_customer(&self, mdoc: i32) -> Result<Option<Customer>, AppError> {
        self.customer_repo.get_by_mdoc(mdoc)
    }

    pub fn search_customers(
        &self,
        page: u32,
        search: Option<String>,
    ) -> Result<Vec<(Customer, i64)>, AppError> {
        let limit = 10;
        let offset = (page.saturating_sub(1) as i64) * limit;
        self.customer_repo.search(limit, offset, search)
    }

    pub fn count_customers(&self, search: Option<String>) -> Result<u32, AppError> {
        self.customer_repo.count(search).map(|c| c as u32)
    }

    pub fn list_club_transactions(&self) -> Result<Vec<ClubTransaction>, AppError> {
        self.tx_repo.list()
    }

    pub fn get_club_transaction(&self, id: i32) -> Result<Option<ClubTransaction>, AppError> {
        self.tx_repo.get_by_id(id)
    }

    pub fn list_club_imports(&self) -> Result<Vec<ClubImport>, AppError> {
        self.import_repo.list()
    }

    pub fn get_club_import(&self, id: i32) -> Result<Option<ClubImport>, AppError> {
        self.import_repo.get_by_id(id)
    }

    pub fn search_club_transactions(
        &self,
        page: u32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(ClubTransaction, Option<String>)>, AppError> {
        let limit = 10;
        let offset = (page.saturating_sub(1) as i64) * limit;
        self.tx_repo.search(limit, offset, date, search)
    }

    pub fn count_club_transactions(
        &self,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<i64, AppError> {
        self.tx_repo.count(date, search)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        mock_club_import_repo::MockClubImportRepo, mock_club_tx_repo::MockClubTransactionRepo,
        mock_customer_repo::MockCustomerRepo,
    };
    use std::sync::Arc;

    fn make_uc() -> ClubUseCases {
        let c_repo = Arc::new(MockCustomerRepo::new());
        let tx_repo = Arc::new(MockClubTransactionRepo::new());
        let im_repo = Arc::new(MockClubImportRepo::new());
        ClubUseCases::new(c_repo, tx_repo, im_repo)
    }

    #[test]
    fn smoke_list_and_get_customers() {
        let uc = make_uc();
        assert!(uc.list_customers().unwrap().is_empty());
        assert!(uc.get_customer(123).unwrap().is_none());
    }

    #[test]
    fn smoke_list_and_get_transactions() {
        let uc = make_uc();
        assert!(uc.list_club_transactions().unwrap().is_empty());
        assert!(uc.get_club_transaction(1).unwrap().is_none());
    }

    #[test]
    fn smoke_list_and_get_imports() {
        let uc = make_uc();
        assert!(uc.list_club_imports().unwrap().is_empty());
        assert!(uc.get_club_import(1).unwrap().is_none());
    }
}
