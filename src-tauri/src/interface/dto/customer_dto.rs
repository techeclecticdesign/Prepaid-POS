use serde::Serialize;

#[derive(Serialize)]
pub struct CustomerReadDto {
    pub mdoc: i32,
    pub name: String,
    pub added: String,
    pub updated: String,
}

#[derive(Serialize)]
pub struct CustomerSearchRow {
    pub customer: CustomerReadDto,
    pub balance: i32,
}

/// The full search result
#[derive(Serialize)]
pub struct CustomerSearchResult {
    pub customers: Vec<CustomerSearchRow>,
    pub total_count: i32,
}

#[derive(Serialize)]
pub struct CustomerPosDto {
    pub customer: CustomerReadDto,
    pub balance: i32,
}
