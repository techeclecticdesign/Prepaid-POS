use crate::interface::common::validators::validate_optional_rfc3339_str;
use validator_derive::Validate;

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct OperatorDto {
    #[validate(range(min = 1, message = "id must be non-zero and positive"))]
    pub id: i32,

    #[validate(length(min = 1, message = "name cannot be empty"))]
    pub name: String,

    #[validate(custom(function = "validate_optional_rfc3339_str"))]
    pub start: Option<String>, // RFC 3339 timestamp

    #[validate(custom(function = "validate_optional_rfc3339_str"))]
    pub stop: Option<String>, // RFC 3339 timestamp
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn valid_operator() {
        let dto = OperatorDto {
            id: 7,
            name: "Alice".into(),
            start: None,
            stop: Some("2025-07-09T12:00:00+00:00".into()),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn invalid_operator_empty_name_and_bad_date() {
        let dto = OperatorDto {
            id: 0,
            name: "".into(),
            start: Some("oops".into()),
            stop: None,
        };
        let err = dto.validate().unwrap_err().to_string();
        assert!(err.contains("id"));
        assert!(err.contains("name"));
        assert!(err.contains("start"));
    }
}
