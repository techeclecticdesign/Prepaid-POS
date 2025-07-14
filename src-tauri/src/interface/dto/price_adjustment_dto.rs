use crate::interface::common::validators::{validate_optional_rfc3339_str, validate_upc_str};
use validator_derive::Validate;

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct PriceAdjustmentDto {
    #[validate(custom(function = "validate_upc_str"))]
    pub upc: String,

    #[validate(range(min = 1, message = "old must be non-zero and positive"))]
    pub old: i32,

    #[validate(range(min = 1, message = "new must be non-zero and positive"))]
    pub new: i32,

    #[validate(range(min = 1, message = "operator_mdoc must be non-zero and positive"))]
    pub operator_mdoc: i32,

    #[validate(custom(function = "validate_optional_rfc3339_str"))]
    pub created_at: Option<String>, // RFC3339
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn valid_price_adjustment() {
        let dto = PriceAdjustmentDto {
            upc: "000000001000".into(),
            old: 500,
            new: 600,
            operator_mdoc: 2,
            created_at: None,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn invalid_price_adjustment_zero_fields() {
        let dto = PriceAdjustmentDto {
            upc: "0".into(),
            old: 0,
            new: 0,
            operator_mdoc: 0,
            created_at: Some("bad".into()),
        };
        let errs = dto.validate().unwrap_err();
        println!("Validation errors: {:?}", errs);

        let err_map = errs.field_errors();
        assert!(err_map.contains_key("upc"));
        assert!(err_map.contains_key("old"));
        assert!(err_map.contains_key("new"));
        assert!(err_map.contains_key("operator_mdoc"));
        assert!(err_map.contains_key("created_at"));

        let created_at_errors = err_map.get("created_at").unwrap();
        assert!(created_at_errors.iter().any(|e| e.code == "rfc3339"));
    }

    #[test]
    fn invalid_price_adjustment_upc_non_digit() {
        // UPC length ok, but letter inside → regex check should fail
        let dto = PriceAdjustmentDto {
            upc: "123B56789012".into(),
            old: 500,
            new: 600,
            operator_mdoc: 2,
            created_at: None,
        };
        let err = dto.validate().unwrap_err().to_string();
        assert!(
            err.contains("upc"),
            "should catch invalid upc with non-digit character"
        );
    }
}
