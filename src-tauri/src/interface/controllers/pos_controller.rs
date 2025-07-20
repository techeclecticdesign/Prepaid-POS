use crate::application::use_cases::pos_usecases::{PosInitData, PosUseCase};
use crate::common::error::AppError;
use crate::domain::repos::{CustomerRepoTrait, ProductRepoTrait};
use crate::interface::dto::{customer_dto::CustomerPosDto, product_dto::ProductDto};
use crate::interface::presenters::customer_presenter::CustomerPresenter;
use crate::interface::presenters::product_presenter::ProductPresenter;
use std::sync::Arc;

pub struct PosController {
    uc: PosUseCase,
}

impl PosController {
    pub fn new(
        product_repo: Arc<dyn ProductRepoTrait>,
        customer_repo: Arc<dyn CustomerRepoTrait>,
    ) -> Self {
        Self {
            uc: PosUseCase::new(product_repo, customer_repo),
        }
    }

    pub fn pos_init(&self) -> Result<(Vec<ProductDto>, Vec<CustomerPosDto>), AppError> {
        let PosInitData {
            products,
            customer_accounts,
        } = self.uc.get_pos_init_data()?;
        let product_dtos = products.into_iter().map(ProductPresenter::to_dto).collect();
        let account_dtos = customer_accounts
            .into_iter()
            .map(|(c, b)| CustomerPresenter::to_pos_dto(c, b))
            .collect();
        Ok((product_dtos, account_dtos))
    }
}
