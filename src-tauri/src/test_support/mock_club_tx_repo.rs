use crate::common::error::AppError;
use crate::domain::models::ClubTransaction;
use crate::domain::repos::ClubTransactionRepoTrait;
use std::sync::Mutex;

pub struct MockClubTransactionRepo {
    store: Mutex<Vec<ClubTransaction>>,
}

impl MockClubTransactionRepo {
    pub fn new() -> Self {
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
}
