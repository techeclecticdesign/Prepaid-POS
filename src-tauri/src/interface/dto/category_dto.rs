use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CategoryDto {
    pub id: i64,
    pub name: String,
    pub deleted: Option<String>, // RFC3339 if soft-deleted
}
