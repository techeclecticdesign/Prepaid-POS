use crate::interface::common::validators::OptionalRfc3339;
use validator_derive::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct CreateCategoryDto {
    #[validate(length(min = 1, message = "category name cannot be empty"))]
    pub name: String,
}

#[derive(serde::Deserialize, Validate)]
pub struct DeleteCategoryDto {
    #[validate(range(min = 1, message = "id must be non-zero and positive"))]
    pub id: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct CategoryDto {
    #[validate(range(min = 1, message = "id must be non-zero and positive"))]
    pub id: i64,

    #[validate(length(min = 1, message = "name cannot be empty"))]
    pub name: String,

    pub deleted: Option<String>, // RFC3339 if soft-deleted
}

impl OptionalRfc3339 for CategoryDto {
    fn optional_dates(&self) -> Vec<(&'static str, &String)> {
        self.deleted
            .as_ref()
            .map(|s| vec![("deleted", s)])
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::common::validators::validate_with_optional_dates;

    #[test]
    fn valid_category() {
        let dto = CategoryDto {
            id: 42,
            name: "Widgets".into(),
            deleted: None,
        };
        assert!(validate_with_optional_dates(&dto).is_ok());
    }

    #[test]
    fn invalid_category_empty_name_and_id_zero() {
        let dto = CategoryDto {
            id: 0,
            name: "".into(),
            deleted: None,
        };
        let err = validate_with_optional_dates(&dto).unwrap_err().to_string();
        assert!(err.contains("id"));
        assert!(err.contains("name"));
    }
}
