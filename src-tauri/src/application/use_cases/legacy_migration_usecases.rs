use crate::common::error::AppError;
use std::path::Path;

pub struct LegacyMigrationUseCases;

impl Default for LegacyMigrationUseCases {
    fn default() -> Self {
        Self::new()
    }
}

impl LegacyMigrationUseCases {
    pub fn new() -> Self {
        Self
    }

    pub fn has_legacy_data(&self) -> Result<bool, AppError> {
        let path = Path::new(r"C:\Annex\CanteenAnnex.accdb");
        Ok(path.exists())
    }
}
