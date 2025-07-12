use crate::common::auth::AuthState;
use crate::common::error::AppError;
use crate::domain::models::ClubImport;
use crate::interface::controllers::parse_pdf_controller::PdfParseController;
use std::sync::{Arc, RwLock};
use tauri::State;

#[tauri::command]
pub fn parse_pdf(
    auth: State<'_, RwLock<AuthState>>,
    ctrl: State<'_, Arc<PdfParseController>>,
    filename: String,
) -> Result<ClubImport, AppError> {
    // Auth check
    let st = auth
        .read()
        .map_err(|e| AppError::LockPoisoned(e.to_string()))?;

    if !st.logged_in {
        return Err(AppError::Unauthorized);
    }
    let parsed = ctrl.parse_pdf(filename)?;
    Ok(parsed)
}
