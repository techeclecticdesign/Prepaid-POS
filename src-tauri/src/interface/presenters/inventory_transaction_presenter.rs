use crate::domain::models::InventoryTransaction;
use crate::interface::dto::inventory_transaction_dto::InventoryTransactionDto;
use chrono::{TimeZone, Utc};

pub struct InventoryTransactionPresenter;

impl InventoryTransactionPresenter {
    pub fn to_dto(itx: InventoryTransaction) -> InventoryTransactionDto {
        InventoryTransactionDto {
            id: itx.id,
            upc: itx.upc,
            quantity_change: itx.quantity_change,
            reference: itx.reference.unwrap_or_default(),
            operator_mdoc: itx.operator_mdoc,
            customer_mdoc: itx.customer_mdoc,
            ref_order_id: itx.ref_order_id,
            created_at: Utc.from_utc_datetime(&itx.created_at).to_rfc3339(),
        }
    }

    pub fn to_dto_list(itxs: Vec<InventoryTransaction>) -> Vec<InventoryTransactionDto> {
        itxs.into_iter().map(Self::to_dto).collect()
    }
}
