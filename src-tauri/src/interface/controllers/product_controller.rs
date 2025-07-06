use crate::application::use_cases::product_usecases::ProductUseCases;
use crate::common::error::AppError;
use crate::interface::dto::price_adjustment_dto::PriceAdjustmentDto;
use crate::interface::dto::product_dto::{ProductDto, RemoveProductDto};
use crate::interface::presenters::price_adjustment_presenter::PriceAdjustmentPresenter;
use crate::interface::presenters::product_presenter::ProductPresenter;
use std::sync::Arc;

pub struct ProductController {
    uc: ProductUseCases,
}

impl ProductController {
    pub fn new(
        repo: Arc<dyn crate::domain::repos::ProductRepoTrait>,
        price_repo: Arc<dyn crate::domain::repos::PriceAdjustmentRepoTrait>,
        conn: Arc<std::sync::Mutex<rusqlite::Connection>>,
    ) -> Self {
        Self {
            uc: ProductUseCases::new(repo, price_repo, conn),
        }
    }

    pub fn create_product(&self, dto: ProductDto) -> Result<(), AppError> {
        self.uc
            .create_product(dto.upc, dto.desc, dto.category, dto.price)
    }

    pub fn remove_product(&self, dto: RemoveProductDto) -> Result<(), AppError> {
        self.uc.remove_product(dto.upc)
    }

    pub fn list_products(&self) -> Result<Vec<ProductDto>, AppError> {
        let ps = self.uc.list_products()?;
        Ok(ProductPresenter::to_dto_list(ps))
    }

    pub fn list_products_category(&self, category: String) -> Result<Vec<ProductDto>, AppError> {
        let ps = self.uc.list_products_by_category(category)?;
        Ok(ProductPresenter::to_dto_list(ps))
    }

    pub fn price_adjustment(
        &self,
        operator_mdoc: i32,
        upc: i64,
        new_price: i32,
    ) -> Result<PriceAdjustmentDto, AppError> {
        let pa = self.uc.price_adjustment(operator_mdoc, upc, new_price)?;
        Ok(PriceAdjustmentPresenter::to_dto(pa))
    }

    pub fn update_item(
        &self,
        upc: i64,
        desc: Option<String>,
        category: Option<String>,
    ) -> Result<(), AppError> {
        self.uc.update_item(upc, desc, category)
    }

    pub fn list_price_adjust_for_product(
        &self,
        upc: i64,
    ) -> Result<Vec<PriceAdjustmentDto>, AppError> {
        let pas = self.uc.list_price_adjust_for_product(upc)?;
        Ok(PriceAdjustmentPresenter::to_dto_list(pas))
    }

    pub fn list_price_adjust_today(&self) -> Result<Vec<PriceAdjustmentDto>, AppError> {
        let pas = self.uc.list_price_adjust_today()?;
        Ok(PriceAdjustmentPresenter::to_dto_list(pas))
    }

    pub fn list_price_adjust_operator(&self, op: i32) -> Result<Vec<PriceAdjustmentDto>, AppError> {
        let pas = self.uc.list_price_adjust_operator(op)?;
        Ok(PriceAdjustmentPresenter::to_dto_list(pas))
    }

    pub fn list_price_adjust(&self) -> Result<Vec<PriceAdjustmentDto>, AppError> {
        let pas = self.uc.list_price_adjust()?;
        Ok(PriceAdjustmentPresenter::to_dto_list(pas))
    }
}

#[cfg(test)]
mod smoke {
    use super::*;
    use crate::test_support::mock_price_adjustment_repo::MockPriceAdjustmentRepo;
    use crate::test_support::mock_product_repo::MockProductRepo;
    use rusqlite::Connection;
    use std::sync::{Arc, Mutex};

    #[test]
    fn controller_smoke_list_products() {
        let prod_repo = Arc::new(MockProductRepo::new());
        let price_repo = Arc::new(MockPriceAdjustmentRepo::new());
        let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
        let ctrl = ProductController::new(prod_repo.clone(), price_repo.clone(), conn);
        let out = ctrl.list_products().expect("list_products should succeed");
        assert!(out.is_empty());
    }

    #[test]
    fn controller_smoke_list_price_adjust() {
        let prod_repo = Arc::new(MockProductRepo::new());
        let price_repo = Arc::new(MockPriceAdjustmentRepo::new());
        let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
        let ctrl = ProductController::new(prod_repo, price_repo.clone(), conn);
        let out = ctrl
            .list_price_adjust()
            .expect("list_price_adjust should succeed");
        assert!(out.is_empty());
    }
}
