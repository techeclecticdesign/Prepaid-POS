use crate::common::error::AppError;
use std::sync::{Arc, RwLock};
use tauri::State;

use crate::common::auth::AuthState;
use crate::interface::controllers::operator_controller::OperatorController;
use crate::interface::dto::operator_dto::OperatorDto;

#[tauri::command]
pub fn list_operators(
    ctrl: State<'_, Arc<OperatorController>>,
) -> Result<Vec<OperatorDto>, AppError> {
    ctrl.list()
}

#[tauri::command]
pub fn create_operator(
    ctrl: State<'_, Arc<OperatorController>>,
    dto: OperatorDto,
) -> Result<(), AppError> {
    ctrl.create(dto)?;
    Ok(())
}

#[tauri::command]
pub fn update_operator(
    auth: State<'_, RwLock<AuthState>>,
    ctrl: State<'_, Arc<OperatorController>>,
    dto: OperatorDto,
) -> Result<(), AppError> {
    // Auth check
    let st = auth
        .read()
        .map_err(|e| AppError::LockPoisoned(e.to_string()))?;

    if !st.logged_in {
        return Err(AppError::Unauthorized);
    }

    ctrl.update(dto)?;
    Ok(())
}
