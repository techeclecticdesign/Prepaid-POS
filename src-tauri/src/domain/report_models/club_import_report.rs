use chrono::NaiveDateTime;

#[derive(Clone)]
pub struct ClubTransactionWithTotal {
    pub id: i32,
    pub import_id: i32,
    pub entity_name: String,
    pub mdoc: Option<i32>,
    pub tx_type: String,
    pub amount: i32,
    pub date: NaiveDateTime,
    pub running_total: i32,
}

pub struct ClubTransactionRow {
    pub tx: ClubTransactionWithTotal,
    pub running_total: i32,
}

pub struct PeriodTotals {
    pub period_pos_sum: i32,
    pub period_neg_sum: i32,
}
