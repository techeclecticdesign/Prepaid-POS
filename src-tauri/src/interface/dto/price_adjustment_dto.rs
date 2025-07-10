use crate::interface::common::validators::OptionalRfc3339;
use validator_derive::Validate;

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct PriceAdjustmentDto {
    #[validate(range(min = 1, message = "upc must be non-zero and positive"))]
    pub upc: i64,

    #[validate(range(min = 1, message = "old must be non-zero and positive"))]
    pub old: i32,

    #[validate(range(min = 1, message = "new must be non-zero and positive"))]
    pub new: i32,

    #[validate(range(min = 1, message = "operator_mdoc must be non-zero and positive"))]
    pub operator_mdoc: i32,

    pub created_at: Option<String>, // RFC3339
}

impl OptionalRfc3339 for PriceAdjustmentDto {
    fn optional_dates(&self) -> Vec<(&'static str, &String)> {
        self.created_at
            .as_ref()
            .map(|s| vec![("created_at", s)])
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::common::validators::validate_with_optional_dates;

    #[test]
    fn valid_price_adjustment() {
        let dto = PriceAdjustmentDto {
            upc: 1000,
            old: 500,
            new: 600,
            operator_mdoc: 2,
            created_at: None,
        };
        assert!(validate_with_optional_dates(&dto).is_ok());
    }

    #[test]
    fn invalid_price_adjustment_zero_fields() {
        let dto = PriceAdjustmentDto {
            upc: 0,
            old: 0,
            new: 0,
            operator_mdoc: 0,
            created_at: Some("bad".into()),
        };
        let err = validate_with_optional_dates(&dto).unwrap_err().to_string();
        assert!(err.contains("upc"));
        assert!(err.contains("old"));
        assert!(err.contains("new"));
        assert!(err.contains("operator_mdoc"));
        assert!(err.contains("rfc3339"));
    }
}
