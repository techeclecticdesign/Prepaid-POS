use crate::interface::common::validators::validate_upc_str;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateProductDto {
    #[validate(custom(function = "validate_upc_str"))]
    pub upc: String,

    #[validate(length(min = 1, message = "desc cannot be empty"))]
    pub desc: String,

    #[validate(length(min = 1, message = "category cannot be empty"))]
    pub category: String,

    #[validate(range(min = 1, message = "price must be non-zero and positive"))]
    pub price: i32,
}

#[derive(Deserialize, Validate)]
pub struct UpdateProductDto {
    #[validate(custom(function = "validate_upc_str"))]
    pub upc: String,

    #[validate(length(min = 1, message = "desc cannot be empty"))]
    pub desc: String,

    #[validate(length(min = 1, message = "category cannot be empty"))]
    pub category: String,
}

#[derive(Deserialize, Validate)]
pub struct DeleteProductDto {
    #[validate(custom(function = "validate_upc_str"))]
    pub upc: String,
}

#[derive(Serialize)]
pub struct ProductDto {
    pub upc: String,
    pub desc: String,
    pub category: String,
    pub price: i32, // integer cents
}

#[derive(Serialize)]
pub struct ProductSearchRow {
    pub product: ProductDto,
    pub available: i64,
}

#[derive(Serialize)]
pub struct ProductSearchResult {
    pub products: Vec<ProductSearchRow>,
    pub total_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn valid_create_product() {
        let dto = CreateProductDto {
            upc: "00000111".into(),
            desc: "Banana".into(),
            category: "Fruit".into(),
            price: 150,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn invalid_create_product_missing_fields() {
        let dto = CreateProductDto {
            upc: "0".into(),
            desc: "".into(),
            category: "".into(),
            price: 0,
        };
        let err = dto.validate().unwrap_err().to_string();
        assert!(err.contains("upc"));
        assert!(err.contains("desc"));
        assert!(err.contains("category"));
        assert!(err.contains("price"));
    }

    #[test]
    fn invalid_create_product_upc_non_digit() {
        // length is 12, but contains a letter -> length ok, numeric check fails
        let dto = CreateProductDto {
            upc: "12A456789012".into(),
            desc: "Test".into(),
            category: "Cat".into(),
            price: 100,
        };
        let err = dto.validate().unwrap_err().to_string();
        assert!(
            err.contains("upc"),
            "should catch invalid upc with non-digit character"
        );
    }
}
