use crate::common::error::AppError;
use crate::interface::controllers::product_controller::ProductController;
use crate::interface::dto::category_dto::{CategoryDto, CreateCategoryDto, DeleteCategoryDto};
use crate::interface::dto::price_adjustment_dto::{
    PriceAdjustmentDto, PriceAdjustmentSearchResult,
};
use crate::interface::dto::product_dto::{
    CreateProductDto, DeleteProductDto, ProductSearchResult, UpdateProductDto,
};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn create_product(
    controller: State<Arc<ProductController>>,
    dto: CreateProductDto,
) -> Result<(), AppError> {
    controller.create_product(dto)
}

#[tauri::command]
pub fn delete_product(
    controller: State<Arc<ProductController>>,
    dto: DeleteProductDto,
) -> Result<(), AppError> {
    controller.delete_product(dto)
}

#[tauri::command]
pub fn price_adjustment(
    controller: State<Arc<ProductController>>,
    dto: PriceAdjustmentDto,
) -> Result<PriceAdjustmentDto, AppError> {
    controller.price_adjustment(dto)
}

#[tauri::command]
pub fn list_price_adjust(
    controller: State<Arc<ProductController>>,
) -> Result<Vec<PriceAdjustmentDto>, AppError> {
    controller.list_price_adjust()
}

#[tauri::command]
pub fn update_product(
    controller: State<Arc<ProductController>>,
    dto: UpdateProductDto,
) -> Result<(), AppError> {
    controller.update_product(dto)
}

#[tauri::command]
pub fn search_products(
    controller: State<Arc<ProductController>>,
    search: Option<String>,
    category: Option<String>,
    page: Option<u32>,
) -> Result<ProductSearchResult, AppError> {
    let page = page.unwrap_or(1);
    controller.search_products(search, category, page)
}

#[tauri::command]
pub fn search_price_adjustments(
    controller: State<Arc<ProductController>>,
    page: Option<u32>,
    date: Option<String>,
    search: Option<String>,
) -> Result<PriceAdjustmentSearchResult, AppError> {
    let page = page.unwrap_or(1);
    controller.search_price_adjustments(page, date, search)
}

#[tauri::command]
pub fn list_categories(
    controller: State<Arc<ProductController>>,
) -> Result<Vec<CategoryDto>, AppError> {
    controller.list_categories()
}

#[tauri::command]
pub fn delete_category(
    controller: State<Arc<ProductController>>,
    dto: DeleteCategoryDto,
) -> Result<(), AppError> {
    controller.delete_category(dto)
}

#[tauri::command]
pub fn create_category(
    controller: State<Arc<ProductController>>,
    dto: CreateCategoryDto,
) -> Result<(), AppError> {
    controller.create_category(dto)
}
