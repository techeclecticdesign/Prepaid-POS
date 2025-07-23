use crate::common::error::AppError;
use crate::domain::models::Category;
use crate::domain::repos::CategoryRepoTrait;
use std::sync::Mutex;

pub struct MockCategoryRepo {
    store: Mutex<Vec<Category>>,
}

impl MockCategoryRepo {
    #[must_use]
    pub const fn new() -> Self {
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

    fn get_by_id(&self, id: i32) -> Result<Option<Category>, AppError> {
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
            id: (guard.len() as i32) + 1,
            name: c,
            deleted: None,
        };
        guard.push(new_category);
        Ok(())
    }

    fn soft_delete(&self, id: i32) -> Result<(), AppError> {
        let mut guard = self.store.lock().unwrap();
        if let Some(cat) = guard.iter_mut().find(|c| c.id == id) {
            cat.deleted = Some(chrono::Utc::now().naive_utc());
        }
        Ok(())
    }

    fn get_by_name(&self, name: &str) -> Result<Option<Category>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.name == name)
            .cloned())
    }

    fn undelete(&self, id: i32) -> Result<(), AppError> {
        let mut guard = self.store.lock().unwrap();
        if let Some(cat) = guard.iter_mut().find(|c| c.id == id) {
            cat.deleted = None;
        }
        Ok(())
    }
}
