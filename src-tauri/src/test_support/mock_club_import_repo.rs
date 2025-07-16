use crate::common::error::AppError;
use crate::domain::models::ClubImport;
use crate::domain::repos::ClubImportRepoTrait;
use std::sync::Mutex;

pub struct MockClubImportRepo {
    store: Mutex<Vec<ClubImport>>,
}

impl MockClubImportRepo {
    pub fn new() -> Self {
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

    fn search(
        &self,
        limit: i64,
        offset: i64,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<ClubImport>, AppError> {
        let mut items = self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|ci| {
                let date_match = date
                    .as_ref()
                    .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                    .map(|parsed| ci.date.date() == parsed)
                    .unwrap_or(true);

                let search_match = search
                    .as_ref()
                    .map(|s| {
                        let s = s.as_str();
                        ci.id.to_string().contains(s) || ci.source_file.contains(s)
                    })
                    .unwrap_or(true);

                date_match && search_match
            })
            .cloned()
            .collect::<Vec<_>>();

        items.sort_by(|a, b| b.date.cmp(&a.date));
        let start = offset as usize;
        let end = (start + limit as usize).min(items.len());
        Ok(items.get(start..end).unwrap_or(&[]).to_vec())
    }

    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i64, AppError> {
        let count = self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|ci| {
                let date_match = date
                    .as_ref()
                    .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                    .map(|parsed| ci.date.date() == parsed)
                    .unwrap_or(true);

                let search_match = search
                    .as_ref()
                    .map(|s| {
                        let s = s.as_str();
                        ci.id.to_string().contains(s) || ci.source_file.contains(s)
                    })
                    .unwrap_or(true);

                date_match && search_match
            })
            .count();

        Ok(count as i64)
    }
}
