use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct ClubImportReadDto {
    pub id: i32,
    pub date: NaiveDateTime,
    pub activity_from: NaiveDateTime,
    pub activity_to: NaiveDateTime,
    pub source_file: String,
}
