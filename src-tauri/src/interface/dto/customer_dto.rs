use serde::Serialize;

#[derive(Serialize)]
pub struct CustomerReadDto {
    pub mdoc: i32,
    pub name: String,
    pub added: String,
    pub updated: String,
}

#[derive(Serialize)]
pub struct CustomerSearchResult {
    pub customers: Vec<CustomerReadDto>,
    pub total_count: u32,
}
