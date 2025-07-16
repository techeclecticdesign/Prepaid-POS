use crate::domain::models::{ClubImport, ClubTransaction};
use crate::interface::dto::{
    club_import_dto::ClubImportReadDto, club_transaction_dto::ClubTransactionReadDto,
};
use chrono::{TimeZone, Utc};

pub struct ClubPresenter;

impl ClubPresenter {
    pub fn to_import_dto(ci: ClubImport) -> ClubImportReadDto {
        ClubImportReadDto {
            id: ci.id,
            date: Utc.from_utc_datetime(&ci.date).to_rfc3339(),
            activity_from: Utc.from_utc_datetime(&ci.activity_from).to_rfc3339(),
            activity_to: Utc.from_utc_datetime(&ci.activity_to).to_rfc3339(),
            source_file: ci.source_file,
        }
    }

    pub fn to_import_dto_list(cis: Vec<ClubImport>) -> Vec<ClubImportReadDto> {
        cis.into_iter().map(Self::to_import_dto).collect()
    }

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
}
