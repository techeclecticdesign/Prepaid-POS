use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OperatorDto {
    pub id: i32,
    pub name: String,
    pub start: String,        // RFC 3339 timestamp
    pub stop: Option<String>, // RFC 3339 timestamp
}
