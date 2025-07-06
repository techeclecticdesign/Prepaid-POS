use crate::common::error::AppError;
use crate::domain::models::Product;
use crate::domain::repos::ProductRepoTrait;
use std::sync::Mutex;

pub struct MockProductRepo {
    store: Mutex<Vec<Product>>,
}

impl MockProductRepo {
    pub fn new() -> Self {
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
    fn get_price(&self, upc: i64) -> Result<i32, AppError> {
        let guard = self.store.lock().unwrap();
        let p = guard
            .iter()
            .find(|p| p.upc == upc)
            .ok_or_else(|| AppError::NotFound(format!("Product {} not found", upc)))?;
        Ok(p.price)
    }

    fn get_by_upc(&self, upc: i64) -> Result<Option<Product>, AppError> {
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
}
