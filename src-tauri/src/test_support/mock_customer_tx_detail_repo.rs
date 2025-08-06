use crate::common::error::AppError;
use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::domain::models::product::Product;
use crate::domain::models::CustomerTransaction;
use crate::domain::report_models::daily_sales::DailySales;
use crate::domain::report_models::product_sales::ProductSalesByCategory;

use crate::domain::report_models::product_sales::SalesTotals;
use crate::domain::repos::customer_tx_detail_repo_trait::CustomerTxDetailRepoTrait;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::sync::Mutex;

// In‑memory mock for CustomerTxDetailRepoTrait
pub struct MockCustomerTxDetailRepo {
    store: Mutex<Vec<CustomerTxDetail>>,
    products: Mutex<HashMap<String, Product>>,
    data: Mutex<Vec<CustomerTxDetail>>,
    transactions: Mutex<Vec<CustomerTransaction>>,
}

impl MockCustomerTxDetailRepo {
    #[must_use]
    pub fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
            products: Mutex::new(HashMap::new()),
            data: Mutex::new(vec![]),
            transactions: Mutex::new(vec![]),
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
            .map(|d| (d, "product".to_string()))
            .collect())
    }

    fn create_with_tx(
        &self,
        d: &CustomerTxDetail,
        _tx: &rusqlite::Transaction<'_>,
    ) -> Result<i32, AppError> {
        self.create(d).map(|_| d.detail_id)
    }

    fn sales_by_category(
        &self,
        _start: NaiveDateTime,
        _end: NaiveDateTime,
    ) -> Result<Vec<ProductSalesByCategory>, AppError> {
        let mut map: HashMap<(String, String), ProductSalesByCategory> = HashMap::new();
        for tx in self.data.lock().unwrap().iter() {
            let products = self.products.lock().unwrap();
            let Some(prod) = products.get(&tx.upc) else {
                continue;
            };
            let key = (prod.category.clone(), tx.upc.clone());
            let entry = map.entry(key.clone()).or_insert(ProductSalesByCategory {
                category: prod.category.clone(),
                upc: tx.upc.clone(),
                name: prod.desc.clone(),
                quantity_sold: 0,
                price: tx.price,
                total_sales: 0,
                is_summary: false,
            });
            entry.quantity_sold += tx.quantity;
            entry.total_sales += tx.quantity * tx.price;
        }
        Ok(map.into_values().collect())
    }

    fn get_sales_totals(
        &self,
        _start: NaiveDateTime,
        _end: NaiveDateTime,
    ) -> Result<SalesTotals, AppError> {
        let total_quantity = self.data.lock().unwrap().iter().map(|tx| tx.quantity).sum();
        let total_value = self
            .data
            .lock()
            .unwrap()
            .iter()
            .map(|tx| tx.quantity * tx.price)
            .sum();
        Ok(SalesTotals {
            total_quantity,
            total_value,
        })
    }

    fn sales_by_day(
        &self,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<DailySales>, AppError> {
        // 1) Build a map from order_id -> date (NaiveDateTime)
        let txs = self.transactions.lock().unwrap();
        let mut date_map: HashMap<i32, NaiveDateTime> = HashMap::new();
        for tx in txs.iter() {
            // only include those in the requested window
            if let Some(tx_date) = tx.date {
                if tx_date >= start && tx_date < end {
                    date_map.insert(tx.order_id, tx_date);
                }
            }
        }
        drop(txs);

        // 2) Sum up `quantity * price` per day
        let mut daily_totals: HashMap<_, i32> = HashMap::new();
        for detail in self.data.lock().unwrap().iter() {
            if let Some(&tx_date) = date_map.get(&detail.order_id) {
                // group by date (YYYY-MM-DD)
                let day = tx_date.date();
                let amount = detail.quantity * detail.price;
                *daily_totals.entry(day).or_insert(0) += amount;
            }
        }

        // 3) Turn into a sorted Vec<DailySales>
        let mut result: Vec<DailySales> = daily_totals
            .into_iter()
            .map(|(day, total_sales)| DailySales { day, total_sales })
            .collect();
        result.sort_by_key(|r| r.day);
        Ok(result)
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
