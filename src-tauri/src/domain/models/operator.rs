use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Operator {
    pub id: i32,
    pub name: String,
    pub start: Option<NaiveDateTime>,
    pub stop: Option<NaiveDateTime>,
}
