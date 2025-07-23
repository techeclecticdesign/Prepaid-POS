use crate::application::use_cases::legacy_migration_usecases::LegacyMigrationDeps;
use crate::application::use_cases::legacy_migration_usecases::LegacyMigrationUseCases;
use crate::common::error::AppError;

pub struct LegacyMigrationController {
    uc: LegacyMigrationUseCases,
}

impl LegacyMigrationController {
    #[must_use]
    pub const fn new(deps: LegacyMigrationDeps) -> Self {
        let uc = LegacyMigrationUseCases::new(deps);
        Self { uc }
    }

    pub fn has_legacy_data(&self) -> Result<bool, AppError> {
        self.uc.has_legacy_data()
    }

    pub fn do_legacy_data_import(&self) -> Result<bool, AppError> {
        self.uc.do_legacy_data_import()
    }
}
