use crate::interface::common::validators::{validate_optional_rfc3339_str, validate_upc_str};
use validator_derive::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct CreateInventoryTransactionDto {
    #[validate(custom(function = "validate_upc_str"))]
    pub upc: String,

    #[validate(range(min = 1, message = "quantity_change must be non-zero and positive"))]
    pub quantity_change: i32,

    pub reference: Option<String>,

    #[validate(range(min = 1, message = "operator_mdoc must be non-zero and positive"))]
    pub operator_mdoc: i32,

    #[validate(range(min = 1, message = "customer_mdoc must be non-zero and positive"))]
    pub customer_mdoc: Option<i32>,

    #[validate(range(min = 1, message = "ref_order_id must be non-zero and positive"))]
    pub ref_order_id: Option<i32>,

    #[validate(custom(function = "validate_optional_rfc3339_str"))]
    pub created_at: Option<String>, // RFC3339
}

#[derive(serde::Serialize)]
pub struct ReadInventoryTransactionDto {
    pub id: Option<i32>,
    pub upc: String,
    pub quantity_change: i32,
    pub reference: Option<String>,
    pub operator_mdoc: i32,
    pub customer_mdoc: Option<i32>,
    pub ref_order_id: Option<i32>,
    pub created_at: Option<String>, // RFC3339
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn valid_inv_tx() {
        let dto = CreateInventoryTransactionDto {
            upc: "000000000100".into(),
            quantity_change: 5,
            reference: Some("restock".into()),
            operator_mdoc: 1,
            customer_mdoc: None,
            ref_order_id: None,
            created_at: None,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn invalid_inv_tx_zero_qty_and_upc() {
        let dto = CreateInventoryTransactionDto {
            upc: "0".into(),
            quantity_change: 0,
            reference: None,
            operator_mdoc: 0,
            customer_mdoc: Some(0),
            ref_order_id: Some(0),
            created_at: Some("not-a-date".into()),
        };
        let err = dto.validate().unwrap_err().to_string();
        assert!(err.contains("upc"));
        assert!(err.contains("quantity_change"));
        assert!(err.contains("operator_mdoc"));
        assert!(err.contains("customer_mdoc"));
        assert!(err.contains("ref_order_id"));
        assert!(err.contains("created_at"));
    }

    #[test]
    fn invalid_inv_tx_upc_non_digit() {
        // UPC length ok, but contains punctuation → numeric check fails
        let dto = CreateInventoryTransactionDto {
            upc: "123-56789012".into(),
            quantity_change: 5,
            reference: Some("restock".into()),
            operator_mdoc: 1,
            customer_mdoc: None,
            ref_order_id: None,
            created_at: None,
        };
        let err = dto.validate().unwrap_err().to_string();
        assert!(
            err.contains("upc"),
            "should catch invalid upc with non-digit character"
        );
    }
}
