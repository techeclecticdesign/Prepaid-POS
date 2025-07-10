use crate::common::error::AppError;
use crate::domain::models::InventoryTransaction;
use crate::domain::repos::InventoryTransactionRepoTrait;
use std::sync::Mutex;

pub struct MockInventoryTransactionRepo {
    store: Mutex<Vec<InventoryTransaction>>,
}

impl MockInventoryTransactionRepo {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

impl Default for MockInventoryTransactionRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl InventoryTransactionRepoTrait for MockInventoryTransactionRepo {
    fn get_by_id(&self, id: i64) -> Result<Option<InventoryTransaction>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|x| x.id.map(|v| v as i64) == Some(id))
            .cloned())
    }

    fn create(&self, tx: &InventoryTransaction) -> Result<(), AppError> {
        self.store.lock().unwrap().push(tx.clone());
        Ok(())
    }

    fn list_for_product(&self, upc: i64) -> Result<Vec<InventoryTransaction>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|x| x.upc == upc)
            .cloned()
            .collect())
    }

    fn list_for_operator(&self, op: i32) -> Result<Vec<InventoryTransaction>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|x| x.operator_mdoc == op)
            .cloned()
            .collect())
    }

    fn list_for_customer(&self, cust: i32) -> Result<Vec<InventoryTransaction>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|x| x.customer_mdoc == Some(cust))
            .cloned()
            .collect())
    }

    fn list_for_today(&self) -> Result<Vec<InventoryTransaction>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }

    fn list(&self) -> Result<Vec<InventoryTransaction>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }
}
