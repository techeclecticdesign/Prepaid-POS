use crate::common::error::AppError;
use crate::domain::models::Category;
use crate::domain::repos::CategoryRepoTrait;
use std::sync::Mutex;

pub struct MockCategoryRepo {
    store: Mutex<Vec<Category>>,
}

impl MockCategoryRepo {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

impl Default for MockCategoryRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl CategoryRepoTrait for MockCategoryRepo {
    fn list(&self) -> Result<Vec<Category>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }

    fn list_active(&self) -> Result<Vec<Category>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.deleted.is_none())
            .cloned()
            .collect())
    }

    fn get_by_id(&self, id: i64) -> Result<Option<Category>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.id == id)
            .cloned())
    }

    fn create(&self, c: String) -> Result<(), AppError> {
        let mut guard = self.store.lock().unwrap();
        let new_category = Category {
            id: (guard.len() as i64) + 1,
            name: c,
            deleted: None,
        };
        guard.push(new_category);
        Ok(())
    }

    fn soft_delete(&self, id: i64) -> Result<(), AppError> {
        let mut guard = self.store.lock().unwrap();
        if let Some(cat) = guard.iter_mut().find(|c| c.id == id) {
            cat.deleted = Some(chrono::Utc::now().naive_utc());
        }
        Ok(())
    }
}
