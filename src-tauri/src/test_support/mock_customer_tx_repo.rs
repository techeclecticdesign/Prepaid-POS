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

    fn search(
        &self,
        limit: i64,
        offset: i64,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(CustomerTransaction, String, i64)>, AppError> {
        let mut items = self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|ct| {
                let mdoc_match = mdoc.is_none_or(|m| ct.customer_mdoc == m);
                let date_match = date
                    .as_ref()
                    .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                    .map(|parsed| ct.date.map(|dt| dt.date() == parsed).unwrap_or(false))
                    .unwrap_or(true);

                let search_match = search
                    .as_ref()
                    .map(|s| {
                        let s = s.as_str();
                        ct.customer_mdoc.to_string().contains(s)
                            || ct.operator_mdoc.to_string().contains(s)
                            || ct.order_id.to_string().contains(s)
                            || ct.note.as_ref().map(|n| n.contains(s)).unwrap_or(false)
                    })
                    .unwrap_or(true);

                mdoc_match && date_match && search_match
            })
            .cloned()
            .map(|ct| (ct, "operator".to_string(), 0))
            .collect::<Vec<_>>();

        items.sort_by(|a, b| b.0.date.cmp(&a.0.date));
        let start = offset as usize;
        let end = (start + limit as usize).min(items.len());
        Ok(items.get(start..end).unwrap_or(&[]).to_vec())
    }

    fn count(
        &self,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<i64, AppError> {
        let count = self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|ct| {
                let mdoc_match = mdoc.is_none_or(|m| ct.customer_mdoc == m);
                let date_match = date
                    .as_ref()
                    .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                    .map(|parsed| ct.date.map(|dt| dt.date() == parsed).unwrap_or(false))
                    .unwrap_or(true);

                let search_match = search
                    .as_ref()
                    .map(|s| {
                        let s = s.as_str();
                        ct.customer_mdoc.to_string().contains(s)
                            || ct.operator_mdoc.to_string().contains(s)
                            || ct.order_id.to_string().contains(s)
                            || ct.note.as_ref().map(|n| n.contains(s)).unwrap_or(false)
                    })
                    .unwrap_or(true);

                mdoc_match && date_match && search_match
            })
            .count();

        Ok(count as i64)
    }

    fn create_with_tx(
        &self,
        tx: &CustomerTransaction,
        _txn: &rusqlite::Transaction<'_>,
    ) -> Result<(), AppError> {
        self.create(tx)
    }
}
