use crate::application::use_cases::club_usecases::ClubUseCases;
use crate::common::error::AppError;
use crate::interface::dto::club_transaction_dto::ClubTransactionSearchResult;
use crate::interface::dto::{
    club_import_dto::ClubImportReadDto,
    club_transaction_dto::ClubTransactionReadDto,
    customer_dto::{CustomerReadDto, CustomerSearchResult},
};
use crate::interface::presenters::club_import_presenter::ClubImportPresenter;
use crate::interface::presenters::club_transaction_presenter::ClubTransactionPresenter;
use crate::interface::presenters::customer_presenter::CustomerPresenter;
use std::sync::Arc;

pub struct ClubController {
    uc: ClubUseCases,
}

impl ClubController {
    pub fn new(
        customer_repo: Arc<dyn crate::domain::repos::CustomerRepoTrait>,
        tx_repo: Arc<dyn crate::domain::repos::ClubTransactionRepoTrait>,
        import_repo: Arc<dyn crate::domain::repos::ClubImportRepoTrait>,
    ) -> Self {
        Self {
            uc: ClubUseCases::new(customer_repo, tx_repo, import_repo),
        }
    }

    pub fn get_customer(&self, mdoc: i32) -> Result<Option<CustomerReadDto>, AppError> {
        let opt = self.uc.get_customer(mdoc)?;
        Ok(opt.map(CustomerPresenter::to_dto))
    }

    pub fn search_customers(
        &self,
        page: u32,
        search: Option<String>,
    ) -> Result<CustomerSearchResult, AppError> {
        let tuples = self.uc.search_customers(page, search.clone())?;
        let total = self.uc.count_customers(search)?;
        Ok(CustomerSearchResult {
            customers: CustomerPresenter::to_search_rows(tuples),
            total_count: total,
        })
    }

    pub fn list_club_transactions(&self) -> Result<Vec<ClubTransactionReadDto>, AppError> {
        let domains = self.uc.list_club_transactions()?;
        Ok(ClubTransactionPresenter::to_transaction_dto_list(domains))
    }

    pub fn get_club_transaction(
        &self,
        id: i32,
    ) -> Result<Option<ClubTransactionReadDto>, AppError> {
        let opt = self.uc.get_club_transaction(id)?;
        Ok(opt.map(ClubTransactionPresenter::to_transaction_dto))
    }

    pub fn list_club_imports(&self) -> Result<Vec<ClubImportReadDto>, AppError> {
        let domains = self.uc.list_club_imports()?;
        Ok(ClubImportPresenter::to_import_dto_list(domains))
    }

    pub fn get_club_import(&self, id: i32) -> Result<Option<ClubImportReadDto>, AppError> {
        let opt = self.uc.get_club_import(id)?;
        Ok(opt.map(ClubImportPresenter::to_import_dto))
    }

    pub fn search_club_transactions(
        &self,
        page: u32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<ClubTransactionSearchResult, AppError> {
        let tuples = self
            .uc
            .search_club_transactions(page, date.clone(), search.clone())?;
        let total = self.uc.count_club_transactions(date, search)?;
        Ok(ClubTransactionSearchResult {
            items: ClubTransactionPresenter::to_search_rows(tuples),
            total_count: total,
        })
    }
}

#[cfg(test)]
mod smoke {
    use super::*;
    use crate::test_support::{
        mock_club_import_repo::MockClubImportRepo, mock_club_tx_repo::MockClubTransactionRepo,
        mock_customer_repo::MockCustomerRepo,
    };
    use std::sync::Arc;

    fn make_controller() -> ClubController {
        let c_repo = Arc::new(MockCustomerRepo::new());
        let tx_repo = Arc::new(MockClubTransactionRepo::new());
        let im_repo = Arc::new(MockClubImportRepo::new());
        ClubController::new(c_repo, tx_repo, im_repo)
    }

    #[test]
    fn smoke_list_transactions() {
        let ctrl = make_controller();
        let out = ctrl.list_club_transactions().unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn smoke_list_imports() {
        let ctrl = make_controller();
        let out = ctrl.list_club_imports().unwrap();
        assert!(out.is_empty());
    }
}
