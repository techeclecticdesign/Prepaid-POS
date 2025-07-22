use crate::application::use_cases::club_usecases::ClubUseCases;
use crate::common::error::AppError;
use crate::interface::dto::club_transaction_dto::ClubTransactionSearchResult;
use crate::interface::dto::customer_dto::CustomerSearchResult;
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
    ) -> Self {
        Self {
            uc: ClubUseCases::new(customer_repo, tx_repo),
        }
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
