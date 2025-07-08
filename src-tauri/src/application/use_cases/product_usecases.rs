use crate::application::common::db::atomic_tx;
use crate::common::error::AppError;
use crate::domain::models::{Category, PriceAdjustment, Product};
use crate::domain::repos::{CategoryRepoTrait, PriceAdjustmentRepoTrait, ProductRepoTrait};
use chrono::Utc;
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

    pub fn create_product(
        &self,
        upc: i64,
        desc: String,
        category: String,
        price: i32,
    ) -> Result<(), AppError> {
        let maybe_existing = self.repo.get_by_upc(upc)?;
        if let Some(existing) = maybe_existing {
            if existing.deleted.is_some() {
                // Resurrect the product by updating it
                let now = Utc::now().naive_utc();
                let resurrected = Product {
                    upc,
                    desc,
                    category,
                    price,
                    updated: now,
                    added: existing.added, // keep original added date
                    deleted: None,         // un-delete
                };
                return self.repo.update_by_upc(&resurrected);
            } else {
                return Err(AppError::Validation(
                    "Product with this UPC already exists".into(),
                ));
            }
        }
        let now = Utc::now().naive_utc();
        let p = Product {
            upc,
            desc,
            category,
            price,
            updated: now,
            added: now,
            deleted: None,
        };
        self.repo.create(&p)
    }

    pub fn remove_product(&self, upc: i64) -> Result<(), AppError> {
        let mut p = self
            .repo
            .get_by_upc(upc)?
            .ok_or_else(|| AppError::NotFound(format!("Product {} not found", upc)))?;
        p.deleted = Some(Utc::now().naive_utc());
        self.repo.update_by_upc(&p)
    }

    pub fn list_products(&self) -> Result<Vec<Product>, AppError> {
        self.repo.list()
    }

    pub fn list_products_by_category(&self, cat: String) -> Result<Vec<Product>, AppError> {
        Ok(self
            .repo
            .list()?
            .into_iter()
            .filter(|p| p.category == cat)
            .collect())
    }

    pub fn price_adjustment(&self, adj: PriceAdjustment) -> Result<PriceAdjustment, AppError> {
        let mut p = self
            .repo
            .get_by_upc(adj.upc)?
            .ok_or_else(|| AppError::NotFound(format!("Product {} not found", adj.upc)))?;

        let adj_id = atomic_tx(&self.conn, |tx| {
            self.price_repo.create_with_tx(&adj, tx)?;
            p.price = adj.new;
            p.updated = chrono::Utc::now().naive_utc();
            self.repo.update_by_upc_with_tx(&p, tx)?;

            Ok(tx.last_insert_rowid())
        })?;

        // now that TX is committed (and Mutex released), read back the row:
        self.price_repo
            .get_by_id(adj_id)?
            .ok_or_else(|| AppError::Unexpected("failed load price adj".into()))
    }

    pub fn update_item(
        &self,
        upc: i64,
        desc: Option<String>,
        category: Option<String>,
    ) -> Result<(), AppError> {
        if desc.is_none() && category.is_none() {
            return Err(AppError::Unexpected("no fields to update".into()));
        }

        // load existing product
        let mut p = self
            .repo
            .get_by_upc(upc)?
            .ok_or_else(|| AppError::NotFound(format!("Product {} not found", upc)))?;

        // update
        if let Some(d) = desc {
            p.desc = d;
        }
        if let Some(c) = category {
            p.category = c;
        }
        p.updated = Utc::now().naive_utc();

        self.repo.update_by_upc(&p)
    }

    pub fn list_price_adjust_today(&self) -> Result<Vec<PriceAdjustment>, AppError> {
        self.price_repo.list_for_today()
    }

    pub fn list_price_adjust_operator(&self, op: i32) -> Result<Vec<PriceAdjustment>, AppError> {
        self.price_repo.list_for_operator(op)
    }

    pub fn list_price_adjust(&self) -> Result<Vec<PriceAdjustment>, AppError> {
        self.price_repo.list()
    }

    pub fn list_price_adjust_for_product(
        &self,
        upc: i64,
    ) -> Result<Vec<PriceAdjustment>, AppError> {
        self.price_repo.list_for_product(upc)
    }

    pub fn search_products(
        &self,
        search: Option<String>,
        category: Option<String>,
        page: u32,
    ) -> Result<Vec<Product>, AppError> {
        let limit = 10;
        let offset = (page.saturating_sub(1) as i64) * limit;
        self.repo.search(search, category, limit, offset)
    }

    pub fn list_categories(&self) -> Result<Vec<Category>, AppError> {
        self.category_repo.list_active()
    }

    pub fn delete_category(&self, id: i64) -> Result<(), AppError> {
        self.category_repo.soft_delete(id)
    }

    pub fn create_category(&self, cat: String) -> Result<(), AppError> {
        if let Some(existing) = self.category_repo.get_by_name(&cat)? {
            // it was soft‐deleted, undelete it.
            if existing.deleted.is_some() {
                return self.category_repo.undelete(existing.id);
            }
            // otherwise it's already active
            return Err(AppError::Unexpected(format!(
                "Category `{}` already exists",
                cat
            )));
        }
        // create new category
        self.category_repo.create(cat)
    }

    pub fn count_products(
        &self,
        search: Option<String>,
        category: Option<String>,
    ) -> Result<u32, AppError> {
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
            upc: 1,
            desc: "Red apple".into(),
            category: "Fruit".into(),
            price: 100,
            updated: chrono::Utc::now().naive_utc(),
            added: chrono::Utc::now().naive_utc(),
            deleted: None,
        })
        .unwrap();
        repo.create(&Product {
            upc: 2,
            desc: "Green apple".into(),
            category: "Fruit".into(),
            price: 100,
            updated: chrono::Utc::now().naive_utc(),
            added: chrono::Utc::now().naive_utc(),
            deleted: None,
        })
        .unwrap();
        repo.create(&Product {
            upc: 3,
            desc: "Yellow banana".into(),
            category: "Fruit".into(),
            price: 200,
            updated: chrono::Utc::now().naive_utc(),
            added: chrono::Utc::now().naive_utc(),
            deleted: None,
        })
        .unwrap();
        repo.create(&Product {
            upc: 4,
            desc: "Blueberry".into(),
            category: "Berry".into(),
            price: 50,
            updated: chrono::Utc::now().naive_utc(),
            added: chrono::Utc::now().naive_utc(),
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
    fn full_product_crud_and_category_filter() -> anyhow::Result<()> {
        let (uc, _op_repo, _prod_repo) = make_use_cases();

        assert!(uc.list_products()?.is_empty());

        uc.create_product(100, "Widget".into(), "Gadgets".into(), 1000)?;
        uc.create_product(200, "Gizmo".into(), "Gadgets".into(), 2000)?;
        uc.create_product(300, "Thing".into(), "Stuff".into(), 1500)?;

        let all = uc.list_products()?;
        assert_eq!(all.len(), 3);

        // filter by category
        let gadgets = uc.list_products_by_category("Gadgets".into())?;
        assert_eq!(
            gadgets.iter().map(|p| p.upc).collect::<Vec<_>>(),
            vec![100, 200]
        );

        // remove one product
        uc.remove_product(200)?;
        let post = uc.list_products()?;
        assert!(post
            .iter()
            .find(|p| p.upc == 200)
            .unwrap()
            .deleted
            .is_some());

        Ok(())
    }

    #[test]
    fn update_item_changes_only_specified_fields() -> anyhow::Result<()> {
        let (uc, _op_repo, _prod_repo) = make_use_cases();
        uc.create_product(42, "OldDesc".into(), "OldCat".into(), 500)?;

        // update desc
        uc.update_item(42, Some("NewDesc".into()), None)?;
        let p = uc.repo.get_by_upc(42)?.unwrap();
        assert_eq!(p.desc, "NewDesc");
        assert_eq!(p.category, "OldCat");

        // update category
        uc.update_item(42, None, Some("NewCat".into()))?;
        let p2 = uc.repo.get_by_upc(42)?.unwrap();
        assert_eq!(p2.desc, "NewDesc");
        assert_eq!(p2.category, "NewCat");

        // no fields -> error
        let err = uc.update_item(42, None, None).unwrap_err();
        assert!(err.to_string().contains("no fields to update"));
        Ok(())
    }

    #[test]
    fn price_adjustment_round_trip() -> anyhow::Result<()> {
        let (uc, operator_repo, _product_repo) = make_use_cases();

        // make sure the operator exists
        operator_repo.create(&Operator {
            id: 1,
            name: "Cashier".into(),
            start: Utc::now().naive_utc(),
            stop: None,
        })?;

        // product must exist
        uc.create_product(7, "Priced".into(), "Cat".into(), 1234)?;

        // adjust price
        let adj = uc.price_adjustment(PriceAdjustment {
            id: 0,
            operator_mdoc: 1,
            upc: 7,
            old: 1234,
            new: 2000,
            created_at: Utc::now().naive_utc(),
        })?;
        assert_eq!(adj.upc, 7);
        assert_eq!(adj.old, 1234);
        assert_eq!(adj.new, 2000);

        // verify update
        let p = uc.repo.get_by_upc(7)?.unwrap();
        assert_eq!(p.price, 2000);

        // listing adjustments
        let today = uc.list_price_adjust_today()?;
        assert_eq!(today.len(), 1);
        Ok(())
    }

    #[test]
    fn list_price_adjust_for_product_returns_expected_results() -> anyhow::Result<()> {
        let (uc, _op_repo, _prod_repo) = make_use_cases();

        // Seed with two price adjustments for same product
        uc.create_product(1, "Item".into(), "Cat".into(), 1000)?;
        uc.price_adjustment(PriceAdjustment {
            id: 0,
            operator_mdoc: 1,
            upc: 1,
            old: 1000,
            new: 1100,
            created_at: Utc::now().naive_utc(),
        })?;
        uc.price_adjustment(PriceAdjustment {
            id: 0,
            operator_mdoc: 1,
            upc: 1,
            old: 1100,
            new: 1200,
            created_at: Utc::now().naive_utc(),
        })?;

        let list = uc.list_price_adjust_for_product(1)?;
        assert_eq!(list.len(), 2);
        assert!(list.iter().all(|pa| pa.upc == 1));

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
