use crate::interface::common::validators::OptionalRfc3339;
use validator_derive::Validate;

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct OperatorDto {
    #[validate(range(min = 1, message = "id must be non-zero and positive"))]
    pub id: i32,

    #[validate(length(min = 1, message = "name cannot be empty"))]
    pub name: String,

    pub start: Option<String>, // RFC 3339 timestamp

    pub stop: Option<String>, // RFC 3339 timestamp
}

impl OptionalRfc3339 for OperatorDto {
    fn optional_dates(&self) -> Vec<(&'static str, &String)> {
        let mut out = Vec::new();
        if let Some(s) = &self.start {
            out.push(("start", s));
        }
        if let Some(s) = &self.stop {
            out.push(("stop", s));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::common::validators::validate_with_optional_dates;

    #[test]
    fn valid_operator() {
        let dto = OperatorDto {
            id: 7,
            name: "Alice".into(),
            start: None,
            stop: Some("2025-07-09T12:00:00+00:00".into()),
        };
        assert!(validate_with_optional_dates(&dto).is_ok());
    }

    #[test]
    fn invalid_operator_empty_name_and_bad_date() {
        let dto = OperatorDto {
            id: 0,
            name: "".into(),
            start: Some("oops".into()),
            stop: None,
        };
        let err = validate_with_optional_dates(&dto).unwrap_err().to_string();
        assert!(err.contains("id"));
        assert!(err.contains("name"));
        assert!(err.contains("rfc3339"));
    }
}
