use crate::domain::models::PriceAdjustment;
use crate::interface::dto::price_adjustment_dto::PriceAdjustmentDto;
use chrono::{TimeZone, Utc};

pub struct PriceAdjustmentPresenter;

impl PriceAdjustmentPresenter {
    pub fn to_dto(pa: PriceAdjustment) -> PriceAdjustmentDto {
        PriceAdjustmentDto {
            id: pa.id,
            upc: pa.upc,
            old: pa.old,
            new: pa.new,
            operator_mdoc: pa.operator_mdoc,
            created_at: Utc.from_utc_datetime(&pa.created_at).to_rfc3339(),
        }
    }

    pub fn to_dto_list(pas: Vec<PriceAdjustment>) -> Vec<PriceAdjustmentDto> {
        pas.into_iter().map(Self::to_dto).collect()
    }
}
