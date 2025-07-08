use crate::domain::models::Operator;
use crate::interface::dto::operator_dto::OperatorDto;
use chrono::TimeZone;

pub struct OperatorPresenter;

impl OperatorPresenter {
    pub fn to_dto_list(ops: Vec<Operator>) -> Vec<OperatorDto> {
        ops.into_iter()
            .map(|o| OperatorDto {
                id: o.id,
                name: o.name,
                start: o
                    .start
                    .map(|dt| chrono::Utc.from_utc_datetime(&dt).to_rfc3339()),
                stop: o
                    .stop
                    .map(|dt| chrono::Utc.from_utc_datetime(&dt).to_rfc3339()),
            })
            .collect()
    }
}
