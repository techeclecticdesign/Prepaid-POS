use crate::domain::models::Customer;
use crate::interface::dto::customer_dto::CustomerReadDto;
use chrono::{TimeZone, Utc};

pub struct CustomerPresenter;

impl CustomerPresenter {
    pub fn to_dto(c: Customer) -> CustomerReadDto {
        CustomerReadDto {
            mdoc: c.mdoc,
            name: c.name,
            added: Utc.from_utc_datetime(&c.added).to_rfc3339(),
            updated: Utc.from_utc_datetime(&c.updated).to_rfc3339(),
        }
    }

    pub fn to_dto_list(cs: Vec<Customer>) -> Vec<CustomerReadDto> {
        cs.into_iter().map(Self::to_dto).collect()
    }
}
