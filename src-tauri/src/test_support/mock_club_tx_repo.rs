use crate::common::error::AppError;
use crate::domain::models::ClubTransaction;
use crate::domain::report_models::club_import_report::{ClubTransactionWithTotal, PeriodTotals};
use crate::domain::repos::ClubTransactionRepoTrait;
use chrono::NaiveDateTime;
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

    fn create(&self, tx: &ClubTransaction) -> Result<(), AppError> {
        self.store.lock().unwrap().push(tx.clone());
        Ok(())
    }
    fn search(
        &self,
        limit: i32,
        offset: i32,
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
    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i32, AppError> {
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

        Ok(cnt as i32)
    }

    fn get_account_total(&self) -> Result<i32, AppError> {
        let guard = self.store.lock().unwrap();
        let sum: i64 = guard.iter().map(|tx| tx.amount as i64).sum();
        Ok(sum as i32)
    }

    fn get_by_import_id_with_total(
        &self,
        import_id: i32,
        start_date: Option<NaiveDateTime>,
    ) -> Result<Vec<ClubTransactionWithTotal>, AppError> {
        let store = self.store.lock().unwrap();
        let mut filtered: Vec<_> = store
            .iter()
            .filter(|tx| Some(tx.import_id) == Some(import_id))
            .filter(|tx| start_date.is_none_or(|s| tx.date >= s))
            .cloned()
            .collect();

        filtered.sort_by_key(|tx| tx.date);

        let mut running_total = 0;
        let result: Vec<_> = filtered
            .into_iter()
            .map(|tx| {
                running_total += tx.amount;
                ClubTransactionWithTotal {
                    id: tx.id,
                    import_id: tx.import_id,
                    entity_name: tx.entity_name.clone(),
                    mdoc: tx.mdoc,
                    tx_type: format!("{:?}", tx.tx_type),
                    amount: tx.amount,
                    date: tx.date,
                    running_total,
                }
            })
            .collect();

        Ok(result)
    }

    fn get_period_sums_for_import(&self, import_id: i32) -> Result<PeriodTotals, AppError> {
        let store = self.store.lock().unwrap();
        let period_pos_sum = store
            .iter()
            .filter(|tx| tx.import_id == import_id)
            .map(|tx| tx.amount.max(0))
            .sum();
        let period_neg_sum = store
            .iter()
            .filter(|tx| tx.import_id == import_id)
            .map(|tx| tx.amount.min(0))
            .sum();

        Ok(PeriodTotals {
            period_pos_sum,
            period_neg_sum,
        })
    }
}
