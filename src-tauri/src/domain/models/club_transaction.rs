use chrono::NaiveDateTime;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClubTransaction {
    pub id: i32,
    pub import_id: i32,
    pub entity_name: String,
    pub mdoc: Option<i32>,
    pub tx_type: TransactionType,
    pub amount: i32,
    pub date: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
}

impl FromSql for TransactionType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        // gets the text or returns an error
        match value.as_str()? {
            "Deposit" => Ok(Self::Deposit),
            "Withdrawal" => Ok(Self::Withdrawal),
            other => Err(FromSqlError::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid TransactionType: {other}"),
            )))),
        }
    }
}
