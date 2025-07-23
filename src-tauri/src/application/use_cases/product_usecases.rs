use crate::application::common::db::atomic_tx;
use crate::common::error::AppError;
use crate::domain::models::{Category, PriceAdjustment, Product};
use crate::domain::repos::{CategoryRepoTrait, PriceAdjustmentRepoTrait, ProductRepoTrait};
use crate::interface::dto::product_dto::UpdateProductDto;
use chrono::Utc;
use log::{error, info};
use std::sync::{Arc, Mutex};

pub struct ProductUseCases {
    repo: Arc<dyn ProductRepoTrait>,
    price_repo: Arc<dyn PriceAdjustmentRepoTrait>,
    category_repo: Arc<dyn CategoryRepoTrait>,
    conn: Arc<Mutex<rusqlite::Connection>>,
}

impl ProductUseCases {
    pub fn new(
        repo: Arc<dyn ProductRepoTrait>,
        price_repo: Arc<dyn PriceAdjustmentRepoTrait>,
        category_repo: Arc<dyn CategoryRepoTrait>,
        conn: Arc<Mutex<rusqlite::Connection>>,
    ) -> Self {
        Self {
            repo,
            price_repo,
            category_repo,
            conn,
        }
    }

    pub fn create_product(&self, product: Product) -> Result<(), AppError> {
        let maybe_existing = self.repo.get_by_upc(product.upc.clone())?;
        if let Some(existing) = maybe_existing {
            if existing.deleted.is_some() {
                // Resurrect the product by updating it
                let now = Some(Utc::now().naive_utc());
                let resurrected = Product {
                    updated: now,
                    added: existing.added, // keep original added date
                    deleted: None,         // un-delete
                    ..product
                };
                return self.repo.update_by_upc(&resurrected);
            }
            return Err(AppError::Validation(
                "Product with this UPC already exists".into(),
            ));
        }
        let now = Some(Utc::now().naive_utc());
        let new_product = Product {
            updated: now,
            added: now,
            deleted: None,
            ..product
        };
        let res = self.repo.create(&new_product);
        match &res {
            Ok(()) => info!(
                "product created: upc={} desc={}",
                new_product.upc, new_product.desc
            ),
            Err(e) => error!("product create error: upc={} error={e}", new_product.upc),
        }
        res
    }

    pub fn delete_product(&self, upc: String) -> Result<(), AppError> {
        let mut p = self
            .repo
            .get_by_upc(upc.clone())?
            .ok_or_else(|| AppError::NotFound(format!("Product {upc} not found")))?;
        p.deleted = Some(Utc::now().naive_utc());
        let res = self.repo.update_by_upc(&p);
        match &res {
            Ok(()) => info!("product deleted: upc={upc}"),
            Err(e) => error!("product delete error: upc={upc} error={e}"),
        }
        res
    }

    pub fn price_adjustment(&self, adj: PriceAdjustment) -> Result<PriceAdjustment, AppError> {
        let mut p = self
            .repo
            .get_by_upc(adj.upc.clone())?
            .ok_or_else(|| AppError::NotFound(format!("Product {} not found", adj.upc)))?;

        let mut adj = adj;
        adj.created_at = Some(chrono::Utc::now().naive_utc());

        let adj_id = atomic_tx(&self.conn, |tx| {
            self.price_repo.create_with_tx(&adj, tx)?;
            p.price = adj.new;
            p.updated = Some(chrono::Utc::now().naive_utc());
            self.repo.update_by_upc_with_tx(&p, tx)?;

            Ok(tx.last_insert_rowid() as i32)
        })?;

        // now that TX is committed (and Mutex released), read back the row:
        let adj_loaded = self.price_repo.get_by_id(adj_id)?;

        match &adj_loaded {
            Some(a) => log::info!(
                "price adjustment recorded: upc={} old={} new={}",
                a.upc,
                a.old,
                a.new
            ),
            None => log::error!("price adjustment not found after insert: id={adj_id}"),
        }

        adj_loaded.ok_or_else(|| AppError::Unexpected("failed load price adj".into()))
    }

    pub fn update_product(&self, dto: UpdateProductDto) -> Result<(), AppError> {
        // load existing product
        let mut p = self
            .repo
            .get_by_upc(dto.upc.clone())?
            .ok_or_else(|| AppError::NotFound(format!("Product {} not found", dto.upc)))?;

        // update
        p.desc = dto.desc;
        p.category = dto.category;
        p.updated = Some(Utc::now().naive_utc());

        let res = self.repo.update_by_upc(&p);
        match &res {
            Ok(()) => info!(
                "product updated: upc={} desc={} category={}",
                p.upc, p.desc, p.category
            ),
            Err(e) => error!("product update error: upc={} error={e}", p.upc),
        }
        res
    }
    pub fn list_price_adjust(&self) -> Result<Vec<PriceAdjustment>, AppError> {
        self.price_repo.list()
    }

    pub fn search_products(
        &self,
        search: Option<String>,
        category: Option<String>,
        page: i32,
    ) -> Result<Vec<(Product, i32)>, AppError> {
        let limit = 10;
        let offset = page.saturating_sub(1) * limit;
        self.repo.search(search, category, limit, offset)
    }

    pub fn search_price_adjustments(
        &self,
        page: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(PriceAdjustment, String, String)>, AppError> {
        let limit = 10;
        let offset = page.saturating_sub(1) * limit;
        self.price_repo.search(limit, offset, date, search)
    }

    pub fn count_price_adjustments(
        &self,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<i32, AppError> {
        self.price_repo.count(date, search)
    }

    pub fn list_categories(&self) -> Result<Vec<Category>, AppError> {
        self.category_repo.list_active()
    }

    pub fn delete_category(&self, id: i32) -> Result<(), AppError> {
        match self.category_repo.soft_delete(id) {
            Ok(()) => {
                info!("category deleted: id={id}");
                Ok(())
            }
            Err(e) => {
                error!("category delete error: id={id} error={e}");
                Err(e)
            }
        }
    }

    pub fn create_category(&self, cat: String) -> Result<(), AppError> {
        if let Some(existing) = self.category_repo.get_by_name(&cat)? {
            // it was soft‐deleted, undelete it.
            if existing.deleted.is_some() {
                return self.category_repo.undelete(existing.id);
            }
            // otherwise it's already active
            return Err(AppError::Unexpected(format!(
                "Category `{cat}` already exists"
            )));
        }
        // create new category
        let res = self.category_repo.create(cat.clone());
        match &res {
            Ok(()) => info!("category created: name={cat}"),
            Err(e) => error!("category create error: name={cat} error={e}"),
        }
        res
    }

    pub fn count_products(
        &self,
        search: Option<String>,
        category: Option<String>,
    ) -> Result<i32, AppError> {
        self.repo.count(search, category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::use_cases::product_usecases::ProductUseCases;
    use crate::domain::models::Operator;
    use crate::domain::repos::OperatorRepoTrait;
    use crate::test_support::mock_category_repo::MockCategoryRepo;
    use crate::test_support::mock_operator_repo::MockOperatorRepo;
    use crate::test_support::mock_price_adjustment_repo::MockPriceAdjustmentRepo;
    use crate::test_support::mock_product_repo::MockProductRepo;
    use std::sync::Arc;

    impl Default for Product {
        fn default() -> Self {
            Self {
                upc: "0".into(),
                desc: String::new(),
                category: String::new(),
                price: 0,
                updated: None,
                added: None,
                deleted: None,
            }
        }
    }

    fn make_use_cases() -> (ProductUseCases, Arc<MockOperatorRepo>, Arc<MockProductRepo>) {
        let op_repo = Arc::new(MockOperatorRepo::new());
        let prod_repo = Arc::new(MockProductRepo::new());
        let price_repo = Arc::new(MockPriceAdjustmentRepo::new());
        let category_repo = Arc::new(MockCategoryRepo::new());
        let conn = Arc::new(Mutex::new(rusqlite::Connection::open_in_memory().unwrap()));
        let uc = ProductUseCases::new(
            prod_repo.clone(),
            price_repo.clone(),
            category_repo.clone(),
            conn,
        );
        (uc, op_repo, prod_repo)
    }

    // populate a few products in a MockProductRepo
    fn make_search_usecase() -> ProductUseCases {
        let repo = Arc::new(MockProductRepo::new());
        let category_repo = Arc::new(MockCategoryRepo::new());
        // insert 4 products
        repo.create(&Product {
            upc: "1".into(),
            desc: "Red apple".into(),
            category: "Fruit".into(),
            price: 100,
            updated: Some(chrono::Utc::now().naive_utc()),
            added: Some(chrono::Utc::now().naive_utc()),
            deleted: None,
        })
        .unwrap();
        repo.create(&Product {
            upc: "2".into(),
            desc: "Green apple".into(),
            category: "Fruit".into(),
            price: 100,
            updated: Some(chrono::Utc::now().naive_utc()),
            added: Some(chrono::Utc::now().naive_utc()),
            deleted: None,
        })
        .unwrap();
        repo.create(&Product {
            upc: "3".into(),
            desc: "Yellow banana".into(),
            category: "Fruit".into(),
            price: 200,
            updated: Some(chrono::Utc::now().naive_utc()),
            added: Some(chrono::Utc::now().naive_utc()),
            deleted: None,
        })
        .unwrap();
        repo.create(&Product {
            upc: "4".into(),
            desc: "Blueberry".into(),
            category: "Berry".into(),
            price: 50,
            updated: Some(chrono::Utc::now().naive_utc()),
            added: Some(chrono::Utc::now().naive_utc()),
            deleted: None,
        })
        .unwrap();

        // price_repo + conn are unused for search
        let price_repo = Arc::new(
            crate::test_support::mock_price_adjustment_repo::MockPriceAdjustmentRepo::new(),
        );
        let conn = Arc::new(Mutex::new(rusqlite::Connection::open_in_memory().unwrap()));

        ProductUseCases::new(repo, price_repo, category_repo, conn)
    }

    #[test]
    fn update_product_changes_only_specified_fields() -> anyhow::Result<()> {
        let (uc, _op_repo, _prod_repo) = make_use_cases();
        uc.create_product(Product {
            upc: "42".into(),
            desc: "OldDesc".into(),
            category: "OldCat".into(),
            price: 500,
            ..Default::default()
        })?;

        // update desc
        uc.update_product(UpdateProductDto {
            upc: "42".into(),
            desc: "NewDesc".into(),
            category: "OldCat".into(),
        })?;
        let p = uc.repo.get_by_upc("42".into())?.unwrap();
        assert_eq!(p.desc, "NewDesc");
        assert_eq!(p.category, "OldCat");

        // update category
        uc.update_product(UpdateProductDto {
            upc: "42".into(),
            desc: "NewDesc".into(),
            category: "NewCat".into(),
        })?;
        let p2 = uc.repo.get_by_upc("42".into())?.unwrap();
        assert_eq!(p2.desc, "NewDesc");
        assert_eq!(p2.category, "NewCat");

        Ok(())
    }

    #[test]
    fn price_adjustment_round_trip() -> anyhow::Result<()> {
        let (uc, operator_repo, _product_repo) = make_use_cases();

        // make sure the operator exists
        operator_repo.create(&Operator {
            mdoc: 1,
            name: "Cashier".into(),
            start: Some(Utc::now().naive_utc()),
            stop: None,
        })?;

        // product must exist
        uc.create_product(Product {
            upc: "7".into(),
            desc: "Priced".into(),
            category: "Cat".into(),
            price: 1234,
            ..Default::default()
        })?;

        // adjust price
        let adj = uc.price_adjustment(PriceAdjustment {
            id: 0,
            operator_mdoc: 1,
            upc: "7".into(),
            old: 1234,
            new: 2000,
            created_at: Some(Utc::now().naive_utc()),
        })?;
        assert_eq!(adj.upc, "7");
        assert_eq!(adj.old, 1234);
        assert_eq!(adj.new, 2000);

        // verify update
        let p = uc.repo.get_by_upc("7".into())?.unwrap();
        assert_eq!(p.price, 2000);

        Ok(())
    }

    #[test]
    fn search_by_desc_substring_and_pagination() -> anyhow::Result<()> {
        let uc = make_search_usecase();

        // page 0: should find 2 apples
        let apples = uc.search_products(Some("apple".into()), None, 0)?;
        assert_eq!(apples.len(), 2);

        // filter by category “Berry”
        let berries = uc.search_products(None, Some("Berry".into()), 0)?;
        assert_eq!(berries.len(), 1);

        // no match
        let none = uc.search_products(Some("pear".into()), None, 0)?;
        assert!(none.is_empty());

        Ok(())
    }

    #[test]
    fn create_category_adds_new_category() -> anyhow::Result<()> {
        let (uc, _, _) = make_use_cases();

        // Should start empty
        let initial = uc.list_categories()?;
        assert!(initial.is_empty());

        // Create one
        uc.create_category("Snacks".to_string())?;

        // List and verify
        let cats = uc.list_categories()?;
        assert_eq!(cats.len(), 1);
        assert_eq!(cats[0].name, "Snacks");
        assert!(cats[0].deleted.is_none());

        Ok(())
    }
}
