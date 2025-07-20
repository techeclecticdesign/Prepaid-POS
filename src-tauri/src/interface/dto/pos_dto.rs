use crate::interface::dto::customer_dto::CustomerPosDto;
use crate::interface::dto::product_dto::ProductDto;
use serde::Serialize;

#[derive(Serialize)]
pub struct PosDto {
    pub products: Vec<ProductDto>,
    pub customers: Vec<CustomerPosDto>,
}
