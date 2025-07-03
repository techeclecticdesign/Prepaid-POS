use crate::domain::models::Operator;
use chrono::TimeZone;
use serde::{Deserialize, Serialize};

/// The shape your frontend expects.
#[derive(Serialize, Deserialize)]
pub struct OperatorDto {
    pub id: i32,
    pub name: String,
    pub start: String,        // RFC 3339 timestamp string
    pub stop: Option<String>, // RFC 3339 timestamp string
}

pub struct OperatorPresenter;

impl OperatorPresenter {
    pub fn to_dto_list(ops: Vec<Operator>) -> Vec<OperatorDto> {
        ops.into_iter()
            .map(|o| OperatorDto {
                id: o.id,
                name: o.name,
                start: chrono::Utc.from_utc_datetime(&o.start).to_rfc3339(),
                stop: o
                    .stop
                    .map(|dt| chrono::Utc.from_utc_datetime(&dt).to_rfc3339()),
            })
            .collect()
    }
}
