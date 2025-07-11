use validator_derive::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct CreateProductDto {
    #[validate(range(min = 1, message = "upc must be non-zero and positive"))]
    pub upc: i64,

    #[validate(length(min = 1, message = "desc cannot be empty"))]
    pub desc: String,

    #[validate(length(min = 1, message = "category cannot be empty"))]
    pub category: String,

    #[validate(range(min = 1, message = "price must be non-zero and positive"))]
    pub price: i32,
}

#[derive(serde::Deserialize, Validate)]
pub struct UpdateProductDto {
    #[validate(range(min = 1, message = "upc must be non-zero and positive"))]
    pub upc: i64,

    #[validate(length(min = 1, message = "desc cannot be empty"))]
    pub desc: String,

    #[validate(length(min = 1, message = "category cannot be empty"))]
    pub category: String,
}

#[derive(serde::Deserialize, Validate)]
pub struct DeleteProductDto {
    #[validate(range(min = 1, message = "upc must be non-zero and positive"))]
    pub upc: i64,
}

#[derive(serde::Serialize)]
pub struct ProductDto {
    pub upc: i64,
    pub desc: String,
    pub category: String,
    pub price: i32, // integer cents
}

#[derive(serde::Serialize)]
pub struct ProductSearchResult {
    pub products: Vec<ProductDto>,
    pub total_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn valid_create_product() {
        let dto = CreateProductDto {
            upc: 111,
            desc: "Banana".into(),
            category: "Fruit".into(),
            price: 150,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn invalid_create_product_missing_fields() {
        let dto = CreateProductDto {
            upc: 0,
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
}
