use serde::Serialize;

#[derive(Serialize)]
pub struct ClubImportReadDto {
    pub id: i32,
    pub date: String,
    pub activity_from: String,
    pub activity_to: String,
    pub source_file: String,
}

#[derive(Serialize)]
pub struct ClubImportSearchResult {
    pub items: Vec<ClubImportReadDto>,
    pub total_count: u32,
}
