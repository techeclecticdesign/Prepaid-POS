use crate::domain::models::Product;
use crate::interface::dto::product_dto::ProductDto;
use chrono::{TimeZone, Utc};

pub struct ProductPresenter;

impl ProductPresenter {
    pub fn to_dto(p: Product) -> ProductDto {
        ProductDto {
            upc: p.upc,
            desc: p.desc,
            category: p.category,
            price: p.price,
            updated: Utc.from_utc_datetime(&p.updated).to_rfc3339(),
            added: Utc.from_utc_datetime(&p.added).to_rfc3339(),
            deleted: p.deleted.map(|dt| Utc.from_utc_datetime(&dt).to_rfc3339()),
        }
    }

    pub fn to_dto_list(ps: Vec<Product>) -> Vec<ProductDto> {
        ps.into_iter().map(Self::to_dto).collect()
    }
}
