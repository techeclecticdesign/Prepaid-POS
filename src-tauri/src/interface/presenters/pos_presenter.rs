use crate::domain::models::{Customer, Product};
use crate::interface::dto::pos_dto::PosDto;
use crate::interface::presenters::customer_presenter::CustomerPresenter;
use crate::interface::presenters::product_presenter::ProductPresenter;

pub struct PosPresenter;

impl PosPresenter {
    pub fn to_dto(products: Vec<Product>, customers: Vec<(Customer, i32)>) -> PosDto {
        // Reuse existing presenters to map each domain model
        let product_dtos = products.into_iter().map(ProductPresenter::to_dto).collect();
        let customer_dtos = customers
            .into_iter()
            .map(|(c, b)| CustomerPresenter::to_pos_dto(c, b))
            .collect();

        PosDto {
            products: product_dtos,
            customers: customer_dtos,
        }
    }
}
