use crate::domain::models::InventoryTransaction;
use crate::interface::dto::inventory_transaction_dto::ReadInventoryTransactionDto;
use chrono::{TimeZone, Utc};

pub struct InventoryTransactionPresenter;

impl InventoryTransactionPresenter {
    pub fn to_dto(itx: InventoryTransaction) -> ReadInventoryTransactionDto {
        let created_at = itx
            .created_at
            .map(|dt| Utc.from_utc_datetime(&dt).to_rfc3339());
        ReadInventoryTransactionDto {
            id: itx.id,
            upc: itx.upc,
            quantity_change: itx.quantity_change,
            reference: itx.reference,
            operator_mdoc: itx.operator_mdoc,
            customer_mdoc: itx.customer_mdoc,
            ref_order_id: itx.ref_order_id,
            created_at,
        }
    }

    pub fn to_dto_list(itxs: Vec<InventoryTransaction>) -> Vec<ReadInventoryTransactionDto> {
        itxs.into_iter().map(Self::to_dto).collect()
    }
}
