use crate::application::use_cases::product_usecases::ProductUseCases;
use crate::common::error::AppError;
use crate::domain::models::price_adjustment::PriceAdjustment;
use crate::domain::models::product::Product;
use crate::interface::dto::category_dto::{CategoryDto, CreateCategoryDto, DeleteCategoryDto};
use crate::interface::dto::price_adjustment_dto::{
    PriceAdjustmentDto, PriceAdjustmentSearchResult,
};
use crate::interface::dto::product_dto::{
    CreateProductDto, DeleteProductDto, ProductSearchResult, UpdateProductDto,
};
use crate::interface::presenters::category_presenter::CategoryPresenter;
use crate::interface::presenters::price_adjustment_presenter::PriceAdjustmentPresenter;
use crate::interface::presenters::product_presenter::ProductPresenter;
use std::sync::Arc;
use validator::Validate;

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

    pub fn create_product(&self, dto: CreateProductDto) -> Result<(), AppError> {
        dto.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;
        let product = Product {
            upc: dto.upc,
            desc: dto.desc,
            category: dto.category,
            price: dto.price,
            updated: None,
            added: None,
            deleted: None,
        };
        self.uc.create_product(product)
    }

    pub fn delete_product(&self, dto: DeleteProductDto) -> Result<(), AppError> {
        dto.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;
        self.uc.delete_product(dto.upc)
    }

    pub fn price_adjustment(
        &self,
        dto: PriceAdjustmentDto,
    ) -> Result<PriceAdjustmentDto, AppError> {
        dto.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;
        let domain = PriceAdjustment {
            id: 0, // gets assigned during persistence
            operator_mdoc: dto.operator_mdoc,
            upc: dto.upc.clone(),
            old: dto.old,
            new: dto.new,
            created_at: None,
        };

        self.uc.price_adjustment(domain)?;
        Ok(dto)
    }

    pub fn update_product(&self, dto: UpdateProductDto) -> Result<(), AppError> {
        dto.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;
        // map domain in use case. Need missing fields from repo & controller can't talk to repo
        self.uc.update_product(dto)
    }

    pub fn search_products(
        &self,
        search: Option<String>,
        category: Option<String>,
        page: u32,
    ) -> Result<ProductSearchResult, AppError> {
        let tuples = self
            .uc
            .search_products(search.clone(), category.clone(), page)?;

        let products = tuples
            .into_iter()
            .map(|(p, avail)| ProductPresenter::to_search_row(p, avail))
            .collect();

        let total_count = self.uc.count_products(search, category)?;

        Ok(ProductSearchResult {
            products,
            total_count,
        })
    }

    pub fn search_price_adjustments(
        &self,
        page: u32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<PriceAdjustmentSearchResult, AppError> {
        let adjustments = self
            .uc
            .search_price_adjustments(page, date.clone(), search.clone())?;
        let total = self.uc.count_price_adjustments(date, search)?;
        Ok(PriceAdjustmentSearchResult {
            adjustments: PriceAdjustmentPresenter::to_dto_search(adjustments),
            total_count: total,
        })
    }

    pub fn list_price_adjust(&self) -> Result<Vec<PriceAdjustmentDto>, AppError> {
        let pas = self.uc.list_price_adjust()?;
        Ok(PriceAdjustmentPresenter::to_dto_list(pas))
    }

    pub fn list_categories(&self) -> Result<Vec<CategoryDto>, AppError> {
        let cats = self.uc.list_categories()?;
        Ok(CategoryPresenter::to_dto_list(cats))
    }
    pub fn delete_category(&self, dto: DeleteCategoryDto) -> Result<(), AppError> {
        dto.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;
        self.uc.delete_category(dto.id)
    }

    pub fn create_category(&self, dto: CreateCategoryDto) -> Result<(), AppError> {
        dto.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;
        self.uc.create_category(dto.name)
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
