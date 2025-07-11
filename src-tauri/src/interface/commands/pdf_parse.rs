use crate::common::auth::AuthState;
use crate::common::error::AppError;
use crate::domain::models::ParsedPdf;
use crate::interface::controllers::pdf_parse_controller::PdfParseController;
use crate::interface::dto::pdf_parse_dto::PdfParseDto;
use std::sync::{Arc, RwLock};
use tauri::State;

#[tauri::command]
pub fn parse_pdf(
    auth: State<'_, RwLock<AuthState>>,
    ctrl: State<'_, Arc<PdfParseController>>,
    dto: PdfParseDto,
) -> Result<ParsedPdf, AppError> {
    // Auth check
    let st = auth.read().unwrap();
    if !st.logged_in {
        return Err(AppError::Unauthorized);
    }
    let parsed = ctrl.parse(dto)?;
    Ok(parsed)
}
