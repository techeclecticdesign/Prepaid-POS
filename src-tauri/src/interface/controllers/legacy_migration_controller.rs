use crate::application::use_cases::legacy_migration_usecases::LegacyMigrationUseCases;
use crate::common::error::AppError;

pub struct LegacyMigrationController {
    uc: LegacyMigrationUseCases,
}

impl Default for LegacyMigrationController {
    fn default() -> Self {
        Self::new()
    }
}

impl LegacyMigrationController {
    pub fn new() -> Self {
        Self {
            uc: LegacyMigrationUseCases::new(),
        }
    }

    pub fn has_legacy_data(&self) -> Result<bool, AppError> {
        self.uc.has_legacy_data()
    }
}
