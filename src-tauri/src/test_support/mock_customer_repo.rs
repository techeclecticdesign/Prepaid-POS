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

    fn update_updated_date(
        &self,
        mdoc: i32,
        new_updated: chrono::NaiveDateTime,
    ) -> Result<(), AppError> {
        let mut store = self.store.lock().unwrap();
        if let Some(c) = store.iter_mut().find(|c| c.mdoc == mdoc) {
            c.updated = new_updated;
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Customer {} not found", mdoc)))
        }
    }
}
