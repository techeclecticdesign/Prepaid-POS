use crate::interface::common::validators::validate_optional_rfc3339_str;
use validator_derive::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct CreateCategoryDto {
    #[validate(length(min = 1, message = "category name cannot be empty"))]
    pub name: String,
}

#[derive(serde::Deserialize, Validate)]
pub struct DeleteCategoryDto {
    #[validate(range(min = 1, message = "id must be non-zero and positive"))]
    pub id: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct CategoryDto {
    #[validate(range(min = 1, message = "id must be non-zero and positive"))]
    pub id: i32,

    #[validate(length(min = 1, message = "name cannot be empty"))]
    pub name: String,

    #[validate(custom(function = "validate_optional_rfc3339_str"))]
    pub deleted: Option<String>, // RFC3339 if soft-deleted
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn valid_category() {
        let dto = CategoryDto {
            id: 42,
            name: "Widgets".into(),
            deleted: None,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn invalid_category_empty_name_and_id_zero() {
        let dto = CategoryDto {
            id: 0,
            name: "".into(),
            deleted: None,
        };
        let err = dto.validate().unwrap_err().to_string();
        assert!(err.contains("id"));
        assert!(err.contains("name"));
    }
}
