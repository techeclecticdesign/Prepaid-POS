use crate::common::error::AppError;
use crate::domain::models::Customer;
use crate::domain::repos::CustomerRepoTrait;
use std::sync::Mutex;

pub struct MockCustomerRepo {
    store: Mutex<Vec<Customer>>,
}

impl MockCustomerRepo {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

impl Default for MockCustomerRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomerRepoTrait for MockCustomerRepo {
    fn list(&self) -> Result<Vec<Customer>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }

    fn get_by_mdoc(&self, mdoc: i32) -> Result<Option<Customer>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.mdoc == mdoc)
            .cloned())
    }

    fn update(&self, customer: &Customer) -> Result<(), AppError> {
        let mut store = self.store.lock().unwrap();
        if let Some(existing) = store.iter_mut().find(|c| c.mdoc == customer.mdoc) {
            *existing = customer.clone();
            Ok(())
        } else {
            Err(AppError::NotFound(format!(
                "Customer {} not found",
                customer.mdoc
            )))
        }
    }

    fn create(&self, customer: &Customer) -> Result<(), AppError> {
        self.store.lock().unwrap().push(customer.clone());
        Ok(())
    }

    fn search(
        &self,
        limit: i64,
        offset: i64,
        search: Option<String>,
    ) -> Result<Vec<Customer>, AppError> {
        let guard = self.store.lock().unwrap();
        let mut items: Vec<Customer> = guard
            .iter()
            .filter(|c| {
                search
                    .as_ref()
                    .map(|s| {
                        let s = s.as_str();
                        c.mdoc.to_string().contains(s) || c.name.contains(s)
                    })
                    .unwrap_or(true)
            })
            .cloned()
            .collect();

        items.sort_by(|a, b| b.added.cmp(&a.added));
        let start = offset as usize;
        let end = (start + limit as usize).min(items.len());
        Ok(items.get(start..end).unwrap_or(&[]).to_vec())
    }

    fn count(&self, search: Option<String>) -> Result<i64, AppError> {
        let guard = self.store.lock().unwrap();
        let count = guard
            .iter()
            .filter(|c| {
                search
                    .as_ref()
                    .map(|s| {
                        let s = s.as_str();
                        c.mdoc.to_string().contains(s) || c.name.contains(s)
                    })
                    .unwrap_or(true)
            })
            .count();
        Ok(count as i64)
    }
}
