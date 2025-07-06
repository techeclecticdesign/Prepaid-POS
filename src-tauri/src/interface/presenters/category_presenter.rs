use crate::domain::models::Category;
use crate::interface::dto::category_dto::CategoryDto;

pub struct CategoryPresenter;

impl CategoryPresenter {
    pub fn to_dto(cat: Category) -> CategoryDto {
        CategoryDto {
            id: cat.id,
            name: cat.name,
            deleted: cat.deleted.map(|dt| dt.format("%+").to_string()),
        }
    }

    pub fn to_dto_list(cats: Vec<Category>) -> Vec<CategoryDto> {
        cats.into_iter().map(Self::to_dto).collect()
    }
}
