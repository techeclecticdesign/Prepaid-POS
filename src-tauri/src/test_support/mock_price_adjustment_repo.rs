use crate::common::error::AppError;
use crate::domain::models::PriceAdjustment;
use crate::domain::repos::PriceAdjustmentRepoTrait;
use std::sync::Mutex;

pub struct MockPriceAdjustmentRepo {
    store: Mutex<Vec<PriceAdjustment>>,
}

impl MockPriceAdjustmentRepo {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

impl Default for MockPriceAdjustmentRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl PriceAdjustmentRepoTrait for MockPriceAdjustmentRepo {
    fn create(&self, adj: &PriceAdjustment) -> Result<(), AppError> {
        self.store.lock().unwrap().push(adj.clone());
        Ok(())
    }

    fn create_with_tx(
        &self,
        adj: &PriceAdjustment,
        _tx: &rusqlite::Transaction<'_>,
    ) -> Result<(), AppError> {
        self.create(adj)
    }

    fn get_by_id(&self, id: i64) -> Result<Option<PriceAdjustment>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|x| x.id == id as i32)
            .cloned())
    }

    fn list_for_product(&self, upc: String) -> Result<Vec<PriceAdjustment>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|x| x.upc == upc)
            .cloned()
            .collect())
    }

    fn list_for_operator(&self, op: i32) -> Result<Vec<PriceAdjustment>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|x| x.operator_mdoc == op)
            .cloned()
            .collect())
    }

    fn list_for_today(&self) -> Result<Vec<PriceAdjustment>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }

    fn list(&self) -> Result<Vec<PriceAdjustment>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }
}
