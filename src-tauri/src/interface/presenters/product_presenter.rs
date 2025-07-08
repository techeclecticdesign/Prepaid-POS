use crate::domain::models::Product;
use crate::interface::dto::product_dto::ProductDto;

pub struct ProductPresenter;

impl ProductPresenter {
    pub fn to_dto(p: Product) -> ProductDto {
        ProductDto {
            upc: p.upc,
            desc: p.desc,
            category: p.category,
            price: p.price,
        }
    }

    pub fn to_dto_list(ps: Vec<Product>) -> Vec<ProductDto> {
        ps.into_iter().map(Self::to_dto).collect()
    }
}
