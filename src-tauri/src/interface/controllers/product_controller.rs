use crate::application::use_cases::product_usecases::ProductUseCases;
use crate::common::error::AppError;
use crate::domain::models::price_adjustment::PriceAdjustment;
use crate::interface::dto::category_dto::CategoryDto;
use crate::interface::dto::price_adjustment_dto::PriceAdjustmentDto;
use crate::interface::dto::product_dto::{ProductDto, ProductSearchResult};
use crate::interface::presenters::category_presenter::CategoryPresenter;
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
        category_repo: Arc<dyn crate::domain::repos::CategoryRepoTrait>,
        conn: Arc<std::sync::Mutex<rusqlite::Connection>>,
    ) -> Self {
        Self {
            uc: ProductUseCases::new(repo, price_repo, category_repo, conn),
        }
    }

    pub fn create_product(
        &self,
        upc: i64,
        desc: String,
        category: String,
        price: i32,
    ) -> Result<(), AppError> {
        self.uc.create_product(upc, desc, category, price)
    }

    pub fn remove_product(&self, upc: i64) -> Result<(), AppError> {
        self.uc.remove_product(upc)
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
        dto: PriceAdjustmentDto,
    ) -> Result<PriceAdjustmentDto, AppError> {
        let domain = PriceAdjustment {
            id: 0, // gets assigned during persistence
            operator_mdoc: dto.operator_mdoc,
            upc: dto.upc,
            old: dto.old,
            new: dto.new,
            created_at: chrono::Utc::now().naive_utc(),
        };

        let pa = self.uc.price_adjustment(domain)?;
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

    pub fn search_products(
        &self,
        search: Option<String>,
        category: Option<String>,
        page: u32,
    ) -> Result<ProductSearchResult, AppError> {
        let products = self
            .uc
            .search_products(search.clone(), category.clone(), page)?;
        let total_count = self.uc.count_products(search, category)?;
        Ok(ProductSearchResult {
            products: ProductPresenter::to_dto_list(products),
            total_count,
        })
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

    pub fn list_categories(&self) -> Result<Vec<CategoryDto>, AppError> {
        let cats = self.uc.list_categories()?;
        Ok(CategoryPresenter::to_dto_list(cats))
    }
    pub fn delete_category(&self, id: i64) -> Result<(), AppError> {
        self.uc.delete_category(id)
    }

    pub fn create_category(&self, cat: String) -> Result<(), AppError> {
        self.uc.create_category(cat)
    }
}

#[cfg(test)]
mod smoke {
    use super::*;
    use crate::test_support::mock_category_repo::MockCategoryRepo;
    use crate::test_support::mock_price_adjustment_repo::MockPriceAdjustmentRepo;
    use crate::test_support::mock_product_repo::MockProductRepo;
    use rusqlite::Connection;
    use std::sync::{Arc, Mutex};

    fn make_controller() -> ProductController {
        let prod_repo = Arc::new(MockProductRepo::new());
        let price_repo = Arc::new(MockPriceAdjustmentRepo::new());
        let category_repo = Arc::new(MockCategoryRepo::new());
        let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
        ProductController::new(prod_repo, price_repo, category_repo, conn)
    }

    #[test]
    fn controller_smoke_list_products() {
        let ctrl = make_controller();
        let out = ctrl.list_products().expect("list_products should succeed");
        assert!(out.is_empty());
    }

    #[test]
    fn controller_smoke_list_price_adjust() {
        let ctrl = make_controller();
        let out = ctrl
            .list_price_adjust()
            .expect("list_price_adjust should succeed");
        assert!(out.is_empty());
    }

    #[test]
    fn controller_smoke_search_products_empty() {
        let ctrl = make_controller();

        // no products added, so search should return empty
        let result = ctrl
            .search_products(Some("apple".into()), Some("Fruit".into()), 0)
            .expect("search_products should succeed");
        assert!(result.products.is_empty());
    }
}
