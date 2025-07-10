use crate::interface::common::date_utils::parse_rfc3339;
use validator::{Validate, ValidationError, ValidationErrors};

pub fn validate_rfc3339(s: &str) -> Result<(), ValidationError> {
    parse_rfc3339(s).map(|_| ()).map_err(|app_err| {
        let mut ve = ValidationError::new("rfc3339");
        ve.message = Some(app_err.to_string().into());
        ve
    })
}

pub trait OptionalRfc3339 {
    fn optional_dates(&self) -> Vec<(&'static str, &String)>;
}

/* Validate all validator rules, then for every `OptionalRfc3339::optional_dates()`,
 * check RFC3339 and add a `"rfc3339"` error on `name`.*/
pub fn validate_with_optional_dates<T>(dto: &T) -> Result<(), ValidationErrors>
where
    T: Validate + OptionalRfc3339,
{
    // collect derive‑macro errors
    let mut errs = match dto.validate() {
        Ok(_) => ValidationErrors::new(),
        Err(e) => e,
    };

    // for each optional date, enforce RFC3339
    for (field, s) in dto.optional_dates() {
        if validate_rfc3339(s).is_err() {
            errs.add(field, ValidationError::new("rfc3339"));
        }
    }

    if errs.is_empty() {
        Ok(())
    } else {
        Err(errs)
    }
}
