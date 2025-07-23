use crate::interface::common::validators::validate_optional_rfc3339_str;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct CustomerTransactionDto {
    #[validate(range(min = 1, message = "order_id must be non-zero and positive"))]
    pub order_id: i32,

    #[validate(range(min = 1, message = "customer_mdoc must be non-zero and positive"))]
    pub customer_mdoc: i32,
    #[validate(range(min = 1, message = "operator_mdoc must be non-zero and positive"))]
    pub operator_mdoc: i32,

    #[validate(custom(function = "validate_optional_rfc3339_str"))]
    pub date: Option<String>, // RFC3339 string, optional

    pub note: Option<String>,
}

#[derive(Serialize)]
pub struct CustomerTransactionSearchResult {
    pub items: Vec<CustomerTransactionSearchRow>,
    pub total_count: i32,
}

#[derive(Serialize)]
pub struct CustomerTransactionSearchRow {
    pub transaction: CustomerTransactionDto,
    pub operator_name: String,
    pub spent: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn valid_txn_minimal() {
        let dto = CustomerTransactionDto {
            order_id: 1,
            customer_mdoc: 5,
            operator_mdoc: 8,
            date: None,
            note: None,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn invalid_txn_bad_ids_and_dates() {
        let dto = CustomerTransactionDto {
            order_id: -1,
            customer_mdoc: 0,
            operator_mdoc: 0,
            date: Some("nope".into()),
            note: None,
        };
        let err = dto.validate().unwrap_err().to_string();
        assert!(err.contains("order_id"));
        assert!(err.contains("customer_mdoc"));
        assert!(err.contains("operator_mdoc"));
        assert!(err.contains("date"));
    }
}
