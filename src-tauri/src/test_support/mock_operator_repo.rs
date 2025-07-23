use crate::common::error::AppError;
use crate::domain::models::Operator;
use crate::domain::repos::OperatorRepoTrait;
use std::sync::Mutex;

/// Shared mock implementation for `OperatorRepoTrait`
pub struct MockOperatorRepo {
    store: Mutex<Vec<Operator>>,
}

impl MockOperatorRepo {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

impl Default for MockOperatorRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl OperatorRepoTrait for MockOperatorRepo {
    fn get_by_mdoc(&self, mdoc: i32) -> Result<Option<Operator>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|o| o.mdoc == mdoc)
            .cloned())
    }
    fn create(&self, operator: &Operator) -> Result<(), AppError> {
        self.store.lock().unwrap().push(operator.clone());
        Ok(())
    }
    fn update_by_mdoc(&self, operator: &Operator) -> Result<(), AppError> {
        let mut guard = self.store.lock().unwrap();
        if let Some(e) = guard.iter_mut().find(|o| o.mdoc == operator.mdoc) {
            *e = operator.clone();
        }
        Ok(())
    }
    fn list(&self) -> Result<Vec<Operator>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }
}
