use crate::common::error::AppError;
use crate::domain::models::ClubImport;
use crate::domain::repos::ClubImportRepoTrait;
use std::sync::Mutex;

pub struct MockClubImportRepo {
    store: Mutex<Vec<ClubImport>>,
}

impl MockClubImportRepo {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

impl Default for MockClubImportRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl ClubImportRepoTrait for MockClubImportRepo {
    fn list(&self) -> Result<Vec<ClubImport>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }

    fn get_by_id(&self, id: i32) -> Result<Option<ClubImport>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|imp| imp.id == id)
            .cloned())
    }

    fn create(&self, import: &ClubImport) -> Result<(), AppError> {
        self.store.lock().unwrap().push(import.clone());
        Ok(())
    }
}
