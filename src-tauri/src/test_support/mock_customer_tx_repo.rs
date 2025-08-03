use crate::common::error::AppError;
use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::domain::models::CustomerTransaction;
use crate::domain::report_models::sales_details::SalesReportDetailRow;
use crate::domain::report_models::sales_details::SalesReportDetails;
use crate::domain::repos::CustomerTransactionRepoTrait;
use chrono::{Duration, NaiveDateTime};
use std::sync::Mutex;

/// Shared mock implementation for `CustomerTransactionRepoTrait`
pub struct MockCustomerTransactionRepo {
    store: Mutex<Vec<CustomerTransaction>>,
}

impl MockCustomerTransactionRepo {
    #[must_use]
    pub const fn new() -> Self {
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
        limit: i32,
        offset: i32,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(CustomerTransaction, String, i32)>, AppError> {
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
                    .is_none_or(|parsed| ct.date.is_some_and(|dt| dt.date() == parsed));

                let search_match = search.as_ref().is_none_or(|s| {
                    let s = s.as_str();
                    ct.customer_mdoc.to_string().contains(s)
                        || ct.operator_mdoc.to_string().contains(s)
                        || ct.order_id.to_string().contains(s)
                        || ct.note.as_ref().is_some_and(|n| n.contains(s))
                });

                mdoc_match && date_match && search_match
            })
            .cloned()
            .map(|ct| (ct, "operator".to_string(), 0_i32))
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
    ) -> Result<i32, AppError> {
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
                    .is_none_or(|parsed| ct.date.is_some_and(|dt| dt.date() == parsed));

                let search_match = search.as_ref().is_none_or(|s| {
                    let s = s.as_str();
                    ct.customer_mdoc.to_string().contains(s)
                        || ct.operator_mdoc.to_string().contains(s)
                        || ct.order_id.to_string().contains(s)
                        || ct.note.as_ref().is_some_and(|n| n.contains(s))
                });

                mdoc_match && date_match && search_match
            })
            .count();

        Ok(count as i32)
    }

    fn create_with_tx(
        &self,
        tx: &CustomerTransaction,
        _txn: &rusqlite::Transaction<'_>,
    ) -> Result<i32, AppError> {
        let mut store = self.store.lock().unwrap();
        let new_id = (store.len() as i32) + 1;
        let mut tx_clone = tx.clone();
        tx_clone.order_id = new_id;
        store.push(tx_clone);
        Ok(new_id)
    }

    fn get_with_details_and_balance(
        &self,
        order_id: i32,
    ) -> Result<(CustomerTransaction, Vec<(CustomerTxDetail, String)>, i32), AppError> {
        let tx = self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|t| t.order_id == order_id)
            .cloned()
            .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;

        let details = vec![(
            CustomerTxDetail {
                detail_id: 1,
                order_id: tx.order_id,
                upc: "123456789012".to_string(),
                quantity: 2,
                price: 300,
            },
            "Test Product".to_string(),
        )];

        let balance = 1234;

        Ok((tx, details, balance))
    }

    fn get_weekly_spent(
        &self,
        customer_mdoc: i32,
        week_start: NaiveDateTime,
    ) -> Result<i32, AppError> {
        let week_end = week_start + Duration::days(7);
        let store = self.store.lock().unwrap();

        let total_spent = store
            .iter()
            .filter(|tx| {
                tx.customer_mdoc == customer_mdoc
                    && tx.date.is_some_and(|d| d >= week_start && d < week_end)
            })
            .map(|tx| {
                let id_str = tx.order_id.to_string();
                if id_str.ends_with('5') {
                    800
                } else {
                    300
                }
            })
            .sum::<i32>();

        Ok(total_spent)
    }

    fn get_sales_details_data(
        &self,
        start: NaiveDateTime,
        _end: NaiveDateTime,
    ) -> Result<Vec<SalesReportDetails>, AppError> {
        let txs = vec![SalesReportDetails {
            tx: CustomerTransaction {
                order_id: 1,
                customer_mdoc: 123,
                operator_mdoc: 888,
                date: Some(start + chrono::Duration::hours(1)),
                note: Some("Mocked transaction".to_string()),
            },
            customer_name: "Test Customer".to_string(),
            item_count: 3,
            order_total: 1500,
            details: vec![
                SalesReportDetailRow {
                    detail_id: 10,
                    order_id: 1,
                    upc: "1111".to_string(),
                    quantity: 1,
                    price: 500,
                    product_name: "Mock Product A".to_string(),
                },
                SalesReportDetailRow {
                    detail_id: 11,
                    order_id: 1,
                    upc: "2222".to_string(),
                    quantity: 2,
                    price: 500,
                    product_name: "Mock Product B".to_string(),
                },
            ],
        }];
        Ok(txs)
    }
}
