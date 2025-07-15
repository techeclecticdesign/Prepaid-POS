use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::interface::dto::customer_tx_detail_dto::CustomerTxDetailDto;

/// Converts domain models into DTOs
pub struct CustomerTxDetailPresenter;

impl CustomerTxDetailPresenter {
    pub fn to_dto(detail: CustomerTxDetail) -> CustomerTxDetailDto {
        CustomerTxDetailDto {
            detail_id: detail.detail_id,
            order_id: detail.order_id,
            upc: detail.upc,
            quantity: detail.quantity,
            price: detail.price,
        }
    }

    pub fn to_dto_list(details: Vec<CustomerTxDetail>) -> Vec<CustomerTxDetailDto> {
        details.into_iter().map(Self::to_dto).collect()
    }
}
