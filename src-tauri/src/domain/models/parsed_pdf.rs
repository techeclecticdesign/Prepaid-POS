use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedPdf {
    pub filename: String,
    pub date: NaiveDateTime,
    pub text: String,
}
