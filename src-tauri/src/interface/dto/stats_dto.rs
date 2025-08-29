use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsDto {
    pub account_total: i64,
    pub total_customer_balances: i64,
}
