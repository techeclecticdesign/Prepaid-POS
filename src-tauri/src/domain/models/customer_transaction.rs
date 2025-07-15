use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CustomerTransaction {
    pub order_id: i32,
    pub customer_mdoc: i32,
    pub operator_mdoc: i32,
    pub date: Option<NaiveDateTime>,
    pub note: Option<String>,
}
