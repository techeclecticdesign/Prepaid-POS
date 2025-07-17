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

    fn list_for_product(&self, upc: String) -> Result<Vec<InventoryTransaction>, AppError> {
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

    fn search(
        &self,
        limit: i64,
        offset: i64,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(InventoryTransaction, String, String)>, AppError> {
        let guard = self.store.lock().unwrap();

        let mut transactions: Vec<(InventoryTransaction, String, String)> = guard
            .iter()
            .filter(|t| {
                // Date match
                let date_match = date
                    .as_ref()
                    .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                    .map(|parsed| t.created_at.map(|dt| dt.date() == parsed).unwrap_or(false))
                    .unwrap_or(true);

                // Search match across multiple fields
                let search_match = search
                    .as_ref()
                    .map(|s| {
                        let s = s.as_str();
                        t.upc.contains(s)
                            || t.operator_mdoc.to_string().contains(s)
                            || t.customer_mdoc
                                .map(|m| m.to_string().contains(s))
                                .unwrap_or(false)
                            || t.ref_order_id
                                .map(|r| r.to_string().contains(s))
                                .unwrap_or(false)
                            || t.reference.as_ref().map(|r| r.contains(s)).unwrap_or(false)
                    })
                    .unwrap_or(true);

                date_match && search_match
            })
            .cloned()
            .map(|t| (t, String::new(), String::new()))
            .collect();

        // Sort newest first
        transactions.sort_by(|a, b| b.0.created_at.cmp(&a.0.created_at));

        // Pagination slice
        let start = offset as usize;
        let end = (start + limit as usize).min(transactions.len());
        Ok(transactions.get(start..end).unwrap_or(&[]).to_vec())
    }

    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i64, AppError> {
        let guard = self.store.lock().unwrap();
        let count = guard
            .iter()
            .filter(|t| {
                let date_match = date
                    .as_ref()
                    .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                    .map(|parsed| t.created_at.map(|dt| dt.date() == parsed).unwrap_or(false))
                    .unwrap_or(true);
                let search_match = search
                    .as_ref()
                    .map(|s| {
                        let s = s.as_str();
                        t.upc.contains(s)
                            || t.operator_mdoc.to_string().contains(s)
                            || t.customer_mdoc
                                .map(|m| m.to_string().contains(s))
                                .unwrap_or(false)
                            || t.ref_order_id
                                .map(|r| r.to_string().contains(s))
                                .unwrap_or(false)
                            || t.reference.as_ref().map(|r| r.contains(s)).unwrap_or(false)
                    })
                    .unwrap_or(true);

                date_match && search_match
            })
            .count();

        Ok(count as i64)
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
