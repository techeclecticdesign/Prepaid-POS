use crate::interface::controllers::auth_controller::AuthController;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn staff_login(
    controller: State<'_, Arc<AuthController>>,
    password: String,
) -> Result<(), String> {
    controller.login(password).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn staff_logout(controller: State<'_, Arc<AuthController>>) -> Result<(), String> {
    controller.logout().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn check_login_status(controller: State<'_, Arc<AuthController>>) -> Result<bool, String> {
    controller.check_status().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_activity(controller: State<'_, Arc<AuthController>>) -> Result<(), String> {
    controller.update_activity().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn change_password(
    controller: State<'_, Arc<AuthController>>,
    old_password: String,
    new_password: String,
) -> Result<(), String> {
    controller
        .change_password(old_password, new_password)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn password_required(controller: State<'_, Arc<AuthController>>) -> Result<bool, String> {
    controller.password_required().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_password(controller: State<'_, Arc<AuthController>>) -> Result<(), String> {
    controller.delete_password().map_err(|e| e.to_string())
}
