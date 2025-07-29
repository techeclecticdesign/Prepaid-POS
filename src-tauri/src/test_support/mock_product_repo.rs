use crate::common::error::AppError;
use crate::domain::models::Product;
use crate::domain::report_models::product_inventory::ProductInventoryReport;
use crate::domain::report_models::product_inventory::ProductInventoryTotals;
use crate::domain::repos::ProductRepoTrait;
use std::sync::Mutex;

pub struct MockProductRepo {
    store: Mutex<Vec<Product>>,
}

impl MockProductRepo {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

impl Default for MockProductRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductRepoTrait for MockProductRepo {
    fn get_price(&self, upc: String) -> Result<i32, AppError> {
        let guard = self.store.lock().unwrap();
        let p = guard
            .iter()
            .find(|p| p.upc == upc)
            .ok_or_else(|| AppError::NotFound(format!("Product {upc} not found")))?;
        Ok(p.price)
    }

    fn get_by_upc(&self, upc: String) -> Result<Option<Product>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.upc == upc)
            .cloned())
    }

    fn create(&self, p: &Product) -> Result<(), AppError> {
        self.store.lock().unwrap().push(p.clone());
        Ok(())
    }

    fn update_by_upc(&self, p: &Product) -> Result<(), AppError> {
        let mut v = self.store.lock().unwrap();
        if let Some(elem) = v.iter_mut().find(|e| e.upc == p.upc) {
            *elem = p.clone();
        }
        Ok(())
    }

    fn update_by_upc_with_tx(
        &self,
        p: &crate::domain::models::Product,
        _tx: &rusqlite::Transaction<'_>,
    ) -> Result<(), AppError> {
        self.update_by_upc(p)
    }

    fn list(&self) -> Result<Vec<Product>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }

    fn search(
        &self,
        desc_like: Option<String>,
        category: Option<String>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<(Product, i32)>, AppError> {
        let guard = self.store.lock().unwrap();
        let mut products: Vec<Product> = guard
            .iter()
            .filter(|p| {
                let desc_match = desc_like.as_ref().is_none_or(|s| p.desc.contains(s));
                let cat_match = category.as_ref().is_none_or(|c| &p.category == c);
                desc_match && cat_match
            })
            .cloned()
            .collect();

        products.sort_by(|a, b| b.added.cmp(&a.added));

        let start = if offset < 0 { 0 } else { offset as usize };
        let limit_usize = if limit < 0 { 0 } else { limit as usize };
        let end = std::cmp::min(start + limit_usize, products.len());
        let sliced = if start < products.len() {
            &products[start..end]
        } else {
            &[]
        };

        let v = sliced.iter().cloned().map(|p| (p, 0_i32)).collect();

        Ok(v)
    }

    fn count(&self, desc_like: Option<String>, category: Option<String>) -> Result<i32, AppError> {
        let guard = self.store.lock().unwrap();
        let count = guard
            .iter()
            .filter(|p| {
                let desc_match = desc_like.as_ref().is_none_or(|s| p.desc.contains(s));
                let cat_match = category.as_ref().is_none_or(|c| &p.category == c);
                desc_match && cat_match
            })
            .count();
        Ok(count as i32)
    }

    fn report_by_category(&self) -> Result<Vec<ProductInventoryReport>, AppError> {
        let products = self.store.lock().unwrap();

        // Group by category
        let mut grouped: std::collections::HashMap<String, Vec<&Product>> =
            std::collections::HashMap::new();
        for p in products.iter().filter(|p| p.deleted.is_none()) {
            grouped.entry(p.category.clone()).or_default().push(p);
        }

        let mut report = vec![];

        for (category, items) in grouped.into_iter() {
            let mut cat_qty = 0;
            let mut cat_total = 0;

            for p in items {
                let quantity = 0; // mocks don't have inventory transactions yet
                let total = p.price * quantity;
                cat_qty += quantity;
                cat_total += total;

                report.push(ProductInventoryReport {
                    category: category.clone(),
                    upc: Some(p.upc.clone()),
                    name: Some(p.desc.clone()),
                    price: Some(p.price),
                    quantity,
                    total,
                    is_summary: false,
                });
            }

            // Add category summary row
            report.push(ProductInventoryReport {
                category,
                upc: None,
                name: None,
                price: None,
                quantity: cat_qty,
                total: cat_total,
                is_summary: true,
            });
        }

        // Sort by category, then is_summary, then name
        report.sort_by(|a, b| {
            a.category
                .cmp(&b.category)
                .then(a.is_summary.cmp(&b.is_summary))
                .then(a.name.cmp(&b.name))
        });

        Ok(report)
    }

    fn get_inventory_totals(&self) -> Result<ProductInventoryTotals, AppError> {
        let products = self.store.lock().unwrap();
        let mut total_quantity = 0;
        let mut total_value = 0;

        for p in products.iter().filter(|p| p.deleted.is_none()) {
            let quantity = 0; // mocks don't have inventory transactions yet
            let total = p.price * quantity;
            total_quantity += quantity;
            total_value += total;
        }

        Ok(ProductInventoryTotals {
            total_quantity,
            total_value,
        })
    }
}
