use crate::domain::models::CustomerTransaction;
use crate::interface::dto::customer_transaction_dto::CustomerTransactionDto;
use chrono::TimeZone;

pub struct CustomerTransactionPresenter;

impl CustomerTransactionPresenter {
    pub fn to_dto_list(txs: Vec<CustomerTransaction>) -> Vec<CustomerTransactionDto> {
        txs.into_iter()
            .map(|t| CustomerTransactionDto {
                order_id: t.order_id,
                customer_mdoc: t.customer_mdoc,
                operator_mdoc: t.operator_mdoc,
                date: t
                    .date
                    .map(|dt| chrono::Utc.from_utc_datetime(&dt).to_rfc3339()),
                note: t.note,
            })
            .collect()
    }

    pub fn to_dto(t: CustomerTransaction) -> CustomerTransactionDto {
        CustomerTransactionDto {
            order_id: t.order_id,
            customer_mdoc: t.customer_mdoc,
            operator_mdoc: t.operator_mdoc,
            date: t
                .date
                .map(|dt| chrono::Utc.from_utc_datetime(&dt).to_rfc3339()),
            note: t.note,
        }
    }
}
