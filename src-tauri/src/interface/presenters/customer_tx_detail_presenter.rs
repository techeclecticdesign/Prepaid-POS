use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::interface::dto::customer_tx_detail_dto::CustomerTxDetailDto;

/// Converts domain models into DTOs
pub struct CustomerTxDetailPresenter;

impl CustomerTxDetailPresenter {
    #[must_use]
    pub fn to_dto(detail: CustomerTxDetail) -> CustomerTxDetailDto {
        CustomerTxDetailDto {
            detail_id: detail.detail_id,
            order_id: detail.order_id,
            upc: detail.upc,
            product_name: String::new(),
            quantity: detail.quantity,
            price: detail.price,
        }
    }

    #[must_use]
    pub fn to_dto_list(rows: Vec<(CustomerTxDetail, String)>) -> Vec<CustomerTxDetailDto> {
        rows.into_iter()
            .map(|(d, product_name)| CustomerTxDetailDto {
                detail_id: d.detail_id,
                order_id: d.order_id,
                upc: d.upc,
                product_name,
                quantity: d.quantity,
                price: d.price,
            })
            .collect()
    }
}
