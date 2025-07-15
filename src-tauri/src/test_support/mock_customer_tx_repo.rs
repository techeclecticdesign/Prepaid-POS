use crate::common::error::AppError;
use crate::domain::models::CustomerTransaction;
use crate::domain::repos::CustomerTransactionRepoTrait;
use std::sync::Mutex;

/// Shared mock implementation for CustomerTransactionRepoTrait
pub struct MockCustomerTransactionRepo {
    store: Mutex<Vec<CustomerTransaction>>,
}

impl MockCustomerTransactionRepo {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

impl Default for MockCustomerTransactionRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomerTransactionRepoTrait for MockCustomerTransactionRepo {
    fn create(&self, tx: &CustomerTransaction) -> Result<(), AppError> {
        self.store.lock().unwrap().push(tx.clone());
        Ok(())
    }

    fn get(&self, order_id: i32) -> Result<Option<CustomerTransaction>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|t| t.order_id == order_id)
            .cloned())
    }

    fn list(&self) -> Result<Vec<CustomerTransaction>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }
}
