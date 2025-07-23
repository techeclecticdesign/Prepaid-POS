use crate::domain::models::Customer;
use crate::interface::dto::customer_dto::{CustomerPosDto, CustomerReadDto, CustomerSearchRow};
use chrono::{TimeZone, Utc};

pub struct CustomerPresenter;

impl CustomerPresenter {
    #[must_use]
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

    #[must_use]
    pub fn to_search_rows(rows: Vec<(Customer, i64)>) -> Vec<CustomerSearchRow> {
        rows.into_iter()
            .map(|(c, balance)| CustomerSearchRow {
                customer: Self::to_dto(c),
                balance,
            })
            .collect()
    }

    #[must_use]
    pub fn to_pos_dto(c: Customer, balance: i32) -> CustomerPosDto {
        CustomerPosDto {
            customer: Self::to_dto(c),
            balance,
        }
    }
}
