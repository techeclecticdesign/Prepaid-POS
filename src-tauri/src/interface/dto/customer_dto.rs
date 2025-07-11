use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct CustomerReadDto {
    pub mdoc: i32,
    pub name: String,
    pub added: NaiveDateTime,
    pub updated: NaiveDateTime,
}
