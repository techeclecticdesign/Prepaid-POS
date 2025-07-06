#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub deleted: Option<chrono::NaiveDateTime>,
}
