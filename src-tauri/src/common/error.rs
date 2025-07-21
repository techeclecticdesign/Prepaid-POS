use std::io;
use tauri::ipc::InvokeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),

    #[error("migration error: {0}")]
    Migration(#[from] rusqlite_migration::Error),

    #[error("chrono parse error: {0}")]
    ChronoParse(#[from] chrono::ParseError),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("mutex was poisoned: {0}")]
    LockPoisoned(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("unexpected error: {0}")]
    Unexpected(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("PDF error: {0}")]
    Pdf(#[from] printpdf::Error),
}

impl From<AppError> for InvokeError {
    fn from(val: AppError) -> InvokeError {
        InvokeError::from(val.to_string())
    }
}
