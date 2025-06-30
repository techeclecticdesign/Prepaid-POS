use crate::domain::models::Operator;
use crate::domain::repos::OperatorRepoTrait;
use crate::error::AppError;
use std::sync::Mutex;

/// Shared mock implementation for OperatorRepoTrait
pub struct MockOperatorRepo {
    store: Mutex<Vec<Operator>>,
}

impl MockOperatorRepo {
    pub fn new() -> Self {
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
    fn get_by_id(&self, id: i32) -> Result<Option<Operator>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|o| o.id == id)
            .cloned())
    }
    fn create(&self, operator: &Operator) -> Result<(), AppError> {
        self.store.lock().unwrap().push(operator.clone());
        Ok(())
    }
    fn update_by_id(&self, operator: &Operator) -> Result<(), AppError> {
        let mut guard = self.store.lock().unwrap();
        if let Some(e) = guard.iter_mut().find(|o| o.id == operator.id) {
            *e = operator.clone();
        }
        Ok(())
    }
    fn list(&self) -> Result<Vec<Operator>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }
}
