use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClubImport {
    pub id: i32,
    pub date: NaiveDateTime,
    pub activity_from: NaiveDateTime,
    pub activity_to: NaiveDateTime,
    pub source_file: String,
}
