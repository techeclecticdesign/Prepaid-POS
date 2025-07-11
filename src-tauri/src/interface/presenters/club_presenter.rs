use crate::domain::models::{ClubImport, ClubTransaction, Customer};
use crate::interface::dto::{
    club_import_dto::ClubImportReadDto, club_transaction_dto::ClubTransactionReadDto,
    customer_dto::CustomerReadDto,
};

pub struct ClubPresenter;

impl ClubPresenter {
    pub fn to_customer_dto(c: Customer) -> CustomerReadDto {
        CustomerReadDto {
            mdoc: c.mdoc,
            name: c.name,
            added: c.added,
            updated: c.updated,
        }
    }

    pub fn to_customer_dto_list(cs: Vec<Customer>) -> Vec<CustomerReadDto> {
        cs.into_iter().map(Self::to_customer_dto).collect()
    }

    pub fn to_transaction_dto(tx: ClubTransaction) -> ClubTransactionReadDto {
        ClubTransactionReadDto {
            id: tx.id,
            mdoc: tx.mdoc,
            tx_type: tx.tx_type,
            amount: tx.amount,
            date: tx.date,
        }
    }

    pub fn to_transaction_dto_list(txs: Vec<ClubTransaction>) -> Vec<ClubTransactionReadDto> {
        txs.into_iter().map(Self::to_transaction_dto).collect()
    }

    pub fn to_import_dto(i: ClubImport) -> ClubImportReadDto {
        ClubImportReadDto {
            id: i.id,
            date: i.date,
            activity_from: i.activity_from,
            activity_to: i.activity_to,
            source_file: i.source_file,
        }
    }

    pub fn to_import_dto_list(is: Vec<ClubImport>) -> Vec<ClubImportReadDto> {
        is.into_iter().map(Self::to_import_dto).collect()
    }
}
