use crate::common::error::AppError;
use crate::domain::models::{ClubTransaction, Customer};
use crate::domain::repos::{ClubTransactionRepoTrait, CustomerRepoTrait};
use std::sync::Arc;

pub struct ClubUseCases {
    customer_repo: Arc<dyn CustomerRepoTrait>,
    tx_repo: Arc<dyn ClubTransactionRepoTrait>,
}

impl ClubUseCases {
    pub fn new(
        customer_repo: Arc<dyn CustomerRepoTrait>,
        tx_repo: Arc<dyn ClubTransactionRepoTrait>,
    ) -> Self {
        Self {
            customer_repo,
            tx_repo,
        }
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
