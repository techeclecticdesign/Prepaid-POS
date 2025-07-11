use crate::domain::models::club_transaction::TransactionType;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct ClubTransactionReadDto {
    pub id: i32,
    pub mdoc: Option<i32>,
    pub tx_type: TransactionType,
    pub amount: i32,
    pub date: NaiveDateTime,
}
