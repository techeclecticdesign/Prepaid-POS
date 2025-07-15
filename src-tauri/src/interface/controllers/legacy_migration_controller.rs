use crate::application::use_cases::legacy_migration_usecases::LegacyMigrationUseCases;
use crate::common::error::AppError;
use crate::domain::repos::{
    CategoryRepoTrait, ClubImportRepoTrait, ClubTransactionRepoTrait, CustomerRepoTrait,
    InventoryTransactionRepoTrait, OperatorRepoTrait, ProductRepoTrait,
};
use std::sync::Arc;

pub struct LegacyMigrationController {
    uc: LegacyMigrationUseCases,
}

impl LegacyMigrationController {
    pub fn new(
        op_repo: Arc<dyn OperatorRepoTrait>,
        product_repo: Arc<dyn ProductRepoTrait>,
        category_repo: Arc<dyn CategoryRepoTrait>,
        customer_repo: Arc<dyn CustomerRepoTrait>,
        club_transaction_repo: Arc<dyn ClubTransactionRepoTrait>,
        club_imports_repo: Arc<dyn ClubImportRepoTrait>,
        inv_repo: Arc<dyn InventoryTransactionRepoTrait>,
    ) -> Self {
        let uc = LegacyMigrationUseCases::new(
            op_repo,
            product_repo,
            category_repo,
            customer_repo,
            club_transaction_repo,
            club_imports_repo,
            inv_repo,
        );
        Self { uc }
    }

    pub fn has_legacy_data(&self) -> Result<bool, AppError> {
        self.uc.has_legacy_data()
    }

    pub fn do_legacy_data_import(&self) -> Result<bool, AppError> {
        self.uc.do_legacy_data_import()
    }
}
