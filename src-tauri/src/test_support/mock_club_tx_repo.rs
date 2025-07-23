use crate::common::error::AppError;
use crate::domain::models::ClubTransaction;
use crate::domain::repos::ClubTransactionRepoTrait;
use std::sync::Mutex;

pub struct MockClubTransactionRepo {
    store: Mutex<Vec<ClubTransaction>>,
}

impl MockClubTransactionRepo {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

impl Default for MockClubTransactionRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl ClubTransactionRepoTrait for MockClubTransactionRepo {
    fn list(&self) -> Result<Vec<ClubTransaction>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }

    fn get_by_id(&self, id: i32) -> Result<Option<ClubTransaction>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|tx| tx.id == id)
            .cloned())
    }

    fn create(&self, tx: &ClubTransaction) -> Result<(), AppError> {
        self.store.lock().unwrap().push(tx.clone());
        Ok(())
    }
    fn search(
        &self,
        limit: i64,
        offset: i64,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(ClubTransaction, Option<String>)>, AppError> {
        let guard = self.store.lock().unwrap();

        // filter by date and by search term over mdoc, entity_name, tx_type, (skip name in mock)
        let mut items: Vec<(ClubTransaction, Option<String>)> = guard
            .iter()
            .filter(|tx| {
                // date filter
                let date_ok = date
                    .as_ref()
                    .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                    .is_none_or(|parsed| tx.date.date() == parsed);

                // search filter
                let search_ok = search.as_ref().is_none_or(|s| {
                    let s = s.as_str();
                    tx.mdoc.is_some_and(|m| m.to_string().contains(s))
                        || tx.entity_name.contains(s)
                        || format!("{:?}", tx.tx_type).contains(s)
                });

                date_ok && search_ok
            })
            .cloned()
            .map(|tx| (tx, None)) // no real customer_name in mock
            .collect();

        // sort descending by date
        items.sort_by(|a, b| b.0.date.cmp(&a.0.date));

        // apply pagination
        let start = offset as usize;
        let end = (start + limit as usize).min(items.len());
        Ok(items.get(start..end).unwrap_or(&[]).to_vec())
    }

    // Count matching entries (ignore pagination)
    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i64, AppError> {
        let guard = self.store.lock().unwrap();
        let cnt = guard
            .iter()
            .filter(|tx| {
                let date_ok = date
                    .as_ref()
                    .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                    .is_none_or(|parsed| tx.date.date() == parsed);

                let search_ok = search.as_ref().is_none_or(|s| {
                    let s = s.as_str();
                    tx.mdoc.is_some_and(|m| m.to_string().contains(s))
                        || tx.entity_name.contains(s)
                        || format!("{:?}", tx.tx_type).contains(s)
                });

                date_ok && search_ok
            })
            .count();

        Ok(cnt as i64)
    }
}
