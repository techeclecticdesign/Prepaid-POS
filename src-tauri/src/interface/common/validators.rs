use crate::interface::common::date_utils::parse_rfc3339;
use validator::ValidationError;

pub fn validate_rfc3339(s: &str) -> Result<(), ValidationError> {
    parse_rfc3339(s).map(|_| ()).map_err(|app_err| {
        let mut ve = ValidationError::new("rfc3339");
        ve.message = Some(app_err.to_string().into());
        ve
    })
}

// Validate an optional RFC3339 string: skip if None, else run validate_rfc3339
pub fn validate_optional_rfc3339_str(s: &str) -> Result<(), ValidationError> {
    validate_rfc3339(s)
}

pub fn validate_upc_str(s: &str) -> Result<(), validator::ValidationError> {
    if !s.chars().all(|c| c.is_ascii_digit()) {
        let mut err = ValidationError::new("invalid_upc");
        err.message = Some("upc must contain only digits".into());
        return Err(err);
    }
    let len = s.len();
    if len != 8 && len != 12 {
        let mut err = ValidationError::new("invalid_upc_length");
        err.message = Some("upc must be 8 or 12 digits".into());
        return Err(err);
    }
    Ok(())
}
