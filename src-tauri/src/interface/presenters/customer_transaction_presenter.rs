use crate::domain::models::CustomerTransaction;
use crate::interface::dto::customer_transaction_dto::{
    CustomerTransactionDto, CustomerTransactionSearchRow,
};
use chrono::TimeZone;

pub struct CustomerTransactionPresenter;

impl CustomerTransactionPresenter {
    #[must_use]
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

    #[must_use]
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

    #[must_use]
    pub fn to_search_rows(
        rows: Vec<(CustomerTransaction, String, i32)>,
    ) -> Vec<CustomerTransactionSearchRow> {
        rows.into_iter()
            .map(|(ct, operator_name, spent)| CustomerTransactionSearchRow {
                transaction: Self::to_dto(ct),
                operator_name,
                spent,
            })
            .collect()
    }
}
