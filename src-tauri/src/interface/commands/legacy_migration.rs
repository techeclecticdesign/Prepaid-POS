use crate::common::error::AppError;
use std::sync::Arc;
use tauri::State;

use crate::interface::controllers::legacy_migration_controller::LegacyMigrationController;

#[tauri::command]
pub fn has_legacy_data(ctrl: State<'_, Arc<LegacyMigrationController>>) -> Result<bool, AppError> {
    let exists = ctrl.has_legacy_data()?;
    Ok(exists)
}
