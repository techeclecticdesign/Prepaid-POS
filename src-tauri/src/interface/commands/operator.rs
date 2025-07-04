use std::sync::Arc;
use tauri::State;

use crate::common::auth::AuthState;
use crate::interface::controllers::operator_controller::OperatorController;
use crate::interface::dto::operator_dto::OperatorDto;
use crate::interface::presenters::operator_presenter::OperatorPresenter;
use std::sync::RwLock;

#[tauri::command]
pub fn list_operators(
    ctrl: State<'_, Arc<OperatorController>>,
) -> Result<Vec<OperatorDto>, String> {
    let ops = ctrl.list().map_err(|e| e.to_string())?;
    Ok(OperatorPresenter::to_dto_list(ops))
}

#[tauri::command]
pub fn get_operator(
    ctrl: State<'_, Arc<OperatorController>>,
    id: i32,
) -> Result<Option<OperatorDto>, String> {
    let opt = ctrl.get(id).map_err(|e| e.to_string())?;
    Ok(opt.map(|o| OperatorPresenter::to_dto_list(vec![o]).remove(0)))
}

#[tauri::command]
pub fn create_operator(
    auth: State<'_, RwLock<AuthState>>,
    ctrl: State<'_, Arc<OperatorController>>,
    dto: OperatorDto,
) -> Result<(), String> {
    // Auth
    {
        let st = auth.read().unwrap();
        if !st.logged_in {
            return Err("Unauthorized".into());
        }
    }
    ctrl.create(dto).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_operator(
    auth: State<'_, RwLock<AuthState>>,
    ctrl: State<'_, Arc<OperatorController>>,
    dto: OperatorDto,
) -> Result<(), String> {
    // Auth
    {
        let st = auth.read().unwrap();
        if !st.logged_in {
            return Err("Unauthorized".into());
        }
    }
    ctrl.update(dto).map_err(|e| e.to_string())
}
