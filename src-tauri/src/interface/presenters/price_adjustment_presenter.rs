use crate::domain::models::PriceAdjustment;
use crate::interface::dto::price_adjustment_dto::PriceAdjustmentDto;
use chrono::{TimeZone, Utc};

pub struct PriceAdjustmentPresenter;

impl PriceAdjustmentPresenter {
    pub fn to_dto(pa: PriceAdjustment) -> PriceAdjustmentDto {
        let created_at = pa
            .created_at
            .map(|dt| Utc.from_utc_datetime(&dt).to_rfc3339());
        PriceAdjustmentDto {
            upc: pa.upc,
            old: pa.old,
            new: pa.new,
            operator_mdoc: pa.operator_mdoc,
            created_at,
        }
    }

    pub fn to_dto_list(pas: Vec<PriceAdjustment>) -> Vec<PriceAdjustmentDto> {
        pas.into_iter().map(Self::to_dto).collect()
    }
}
