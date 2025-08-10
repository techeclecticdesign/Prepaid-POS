use crate::common::error::AppError;
use crate::domain::models::{Customer, Product};
use crate::domain::repos::{CustomerRepoTrait, ProductRepoTrait};
use crate::try_log;
use std::sync::Arc;

pub struct PosInitData {
    pub products: Vec<Product>,
    pub customer_accounts: Vec<(Customer, i32)>,
}

pub struct PosUseCase {
    product_repo: Arc<dyn ProductRepoTrait>,
    customer_repo: Arc<dyn CustomerRepoTrait>,
}

impl PosUseCase {
    pub fn new(
        product_repo: Arc<dyn ProductRepoTrait>,
        customer_repo: Arc<dyn CustomerRepoTrait>,
    ) -> Self {
        Self {
            product_repo,
            customer_repo,
        }
    }

    pub fn get_pos_init_data(&self) -> Result<PosInitData, AppError> {
        let products = try_log!(self.product_repo.list(), "PosUseCase::get_pos_init_data");
        let customer_accounts = try_log!(
            self.customer_repo.list_customer_accounts(),
            "PosUseCase::get_pos_init_data"
        );
        Ok(PosInitData {
            products,
            customer_accounts,
        })
    }
}
