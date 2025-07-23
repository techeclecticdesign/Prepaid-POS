use crate::domain::models::ClubImport;
use crate::interface::dto::club_import_dto::ClubImportReadDto;
use chrono::{TimeZone, Utc};

pub struct ClubImportPresenter;

impl ClubImportPresenter {
    #[must_use]
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
}
