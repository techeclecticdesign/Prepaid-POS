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
        let mut store = self.store.lock().unwrap();
        let mut detail = d.clone();
        if detail.detail_id == 0 {
            // Simulate auto-increment
            let max_id = store.iter().map(|e| e.detail_id).max().unwrap_or(0);
            detail.detail_id = max_id + 1;
        }
        store.push(detail);
        Ok(())
    }

    fn list_by_order(&self, order_id: i32) -> Result<Vec<(CustomerTxDetail, String)>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|d| d.order_id == order_id)
            .cloned()
            .map(|d| (d.clone(), "product".to_string()))
            .collect())
    }

    fn create_with_tx(
        &self,
        d: &CustomerTxDetail,
        _tx: &rusqlite::Transaction<'_>,
    ) -> Result<(), AppError> {
        self.create(d)
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
