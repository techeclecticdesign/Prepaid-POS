use crate::common::error::AppError;
use std::sync::{Arc, RwLock};
use tauri::State;

use crate::common::auth::AuthState;
use crate::interface::controllers::operator_controller::OperatorController;
use crate::interface::dto::operator_dto::OperatorDto;
use crate::interface::presenters::operator_presenter::OperatorPresenter;

#[tauri::command]
pub fn list_operators(
    ctrl: State<'_, Arc<OperatorController>>,
) -> Result<Vec<OperatorDto>, AppError> {
    let ops = ctrl.list()?;
    Ok(OperatorPresenter::to_dto_list(ops))
}

#[tauri::command]
pub fn get_operator(
    ctrl: State<'_, Arc<OperatorController>>,
    id: i32,
) -> Result<Option<OperatorDto>, AppError> {
    let opt = ctrl.get(id)?;
    let dto = opt.map(|o| OperatorPresenter::to_dto_list(vec![o]).remove(0));
    Ok(dto)
}

#[tauri::command]
pub fn create_operator(
    auth: State<'_, RwLock<AuthState>>,
    ctrl: State<'_, Arc<OperatorController>>,
    dto: OperatorDto,
) -> Result<(), AppError> {
    let st = auth.read().unwrap();
    if !st.logged_in {
        return Err(AppError::Unauthorized);
    }

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
    let st = auth.read().unwrap();
    if !st.logged_in {
        return Err(AppError::Unauthorized);
    }

    ctrl.update(dto)?;
    Ok(())
}
