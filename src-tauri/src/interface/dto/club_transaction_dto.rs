use crate::domain::models::club_transaction::TransactionType;
use serde::Serialize;

#[derive(Serialize)]
pub struct ClubTransactionReadDto {
    pub id: i32,
    pub mdoc: Option<i32>,
    pub tx_type: TransactionType,
    pub amount: i32,
    pub date: String,
}

#[derive(Serialize)]
pub struct ClubTransactionSearchResult {
    pub items: Vec<ClubTransactionSearchRow>,
    pub total_count: i32,
}

#[derive(Serialize)]
pub struct ClubTransactionSearchRow {
    pub transaction: ClubTransactionReadDto,
    pub customer_name: Option<String>,
}
