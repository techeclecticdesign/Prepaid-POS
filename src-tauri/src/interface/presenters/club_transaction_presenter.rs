use crate::domain::models::ClubTransaction;
use crate::interface::dto::club_transaction_dto::{
    ClubTransactionReadDto, ClubTransactionSearchRow,
};
use chrono::{TimeZone, Utc};

pub struct ClubTransactionPresenter;

impl ClubTransactionPresenter {
    #[must_use]
    pub fn to_transaction_dto(ct: ClubTransaction) -> ClubTransactionReadDto {
        ClubTransactionReadDto {
            id: ct.id,
            mdoc: ct.mdoc,
            tx_type: ct.tx_type,
            amount: ct.amount,
            date: Utc.from_utc_datetime(&ct.date).to_rfc3339(),
        }
    }

    pub fn to_transaction_dto_list(cts: Vec<ClubTransaction>) -> Vec<ClubTransactionReadDto> {
        cts.into_iter().map(Self::to_transaction_dto).collect()
    }

    #[must_use]
    pub fn to_search_rows(
        rows: Vec<(ClubTransaction, Option<String>)>,
    ) -> Vec<ClubTransactionSearchRow> {
        rows.into_iter()
            .map(|(ct, name)| ClubTransactionSearchRow {
                transaction: Self::to_transaction_dto(ct),
                customer_name: name,
            })
            .collect()
    }
}
