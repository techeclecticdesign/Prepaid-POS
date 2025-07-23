use crate::domain::models::Product;
use crate::interface::dto::product_dto::{ProductDto, ProductSearchRow};

pub struct ProductPresenter;

impl ProductPresenter {
    #[must_use]
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

    #[must_use]
    pub fn to_search_row(p: Product, available: i64) -> ProductSearchRow {
        ProductSearchRow {
            product: Self::to_dto(p),
            available,
        }
    }
}
