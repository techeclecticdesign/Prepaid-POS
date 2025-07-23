use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Customer {
    pub mdoc: i32,
    pub name: String,
    pub added: NaiveDateTime,
    pub updated: NaiveDateTime,
}
