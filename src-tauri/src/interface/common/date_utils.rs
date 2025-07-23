use crate::common::error::AppError;
use chrono::{DateTime, FixedOffset, NaiveDateTime};

/// Parse a required RFC3339 string into a local `NaiveDateTime`.
pub fn parse_rfc3339(s: &str) -> Result<NaiveDateTime, AppError> {
    DateTime::<FixedOffset>::parse_from_rfc3339(s)
        .map_err(|e| AppError::Validation(format!("invalid timestamp: {e}")))
        .map(|dt| dt.naive_local())
}

/// Parse an optional RFC3339 string into Option<NaiveDateTime>.
pub fn parse_optional_rfc3339(opt: &Option<String>) -> Result<Option<NaiveDateTime>, AppError> {
    if let Some(ref s) = opt {
        Ok(Some(parse_rfc3339(s)?))
    } else {
        Ok(None)
    }
}
