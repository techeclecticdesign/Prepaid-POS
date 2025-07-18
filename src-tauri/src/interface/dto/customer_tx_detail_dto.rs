use crate::interface::common::validators::validate_upc_str;
use validator_derive::Validate;

/// DTO for creating a line‑item
#[derive(serde::Deserialize, Validate)]
pub struct CreateCustomerTxDetailDto {
    #[validate(range(min = 1, message = "order_id must be non-zero and positive"))]
    pub order_id: i32,

    #[validate(custom(function = "validate_upc_str"))]
    pub upc: String,

    #[validate(range(min = 1, message = "quantity must be non-zero and positive"))]
    pub quantity: i32,

    #[validate(range(min = 1, message = "price must be non-zero and positive"))]
    pub price: i32,
}

/// DTO for returning a line‑item
#[derive(serde::Serialize)]
pub struct CustomerTxDetailDto {
    pub detail_id: i32,
    pub order_id: i32,
    pub upc: String,
    pub product_name: String,
    pub quantity: i32,
    pub price: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn valid_create_detail() {
        let dto = CreateCustomerTxDetailDto {
            order_id: 1,
            upc: "00000001".into(),
            quantity: 2,
            price: 150,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn invalid_create_detail() {
        let dto = CreateCustomerTxDetailDto {
            order_id: 0,
            upc: "ABC".into(),
            quantity: 0,
            price: 0,
        };
        let err = dto.validate().unwrap_err().to_string();
        assert!(err.contains("order_id"));
        assert!(err.contains("upc"));
        assert!(err.contains("quantity"));
        assert!(err.contains("price"));
    }
}
