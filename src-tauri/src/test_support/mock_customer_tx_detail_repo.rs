use crate::common::error::AppError;
use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::domain::repos::customer_tx_detail_repo_trait::CustomerTxDetailRepoTrait;
use std::sync::Mutex;

// In‑memory mock for CustomerTxDetailRepoTrait
pub struct MockCustomerTxDetailRepo {
    store: Mutex<Vec<CustomerTxDetail>>,
}

impl MockCustomerTxDetailRepo {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }

    // Test helper to list all stored details
    pub fn list_all(&self) -> Result<Vec<CustomerTxDetail>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }
}

impl Default for MockCustomerTxDetailRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomerTxDetailRepoTrait for MockCustomerTxDetailRepo {
    fn create(&self, d: &CustomerTxDetail) -> Result<(), AppError> {
        self.store.lock().unwrap().push(d.clone());
        Ok(())
    }

    fn list_by_order(&self, order_id: i32) -> Result<Vec<CustomerTxDetail>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|d| d.order_id == order_id)
            .cloned()
            .collect())
    }
}

pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
