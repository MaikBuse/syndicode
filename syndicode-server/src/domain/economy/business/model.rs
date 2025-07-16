use bon::Builder;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Builder, Clone, FromRow)]
pub struct Business {
    pub uuid: Uuid,
    pub market_uuid: Uuid,
    pub owning_corporation_uuid: Option<Uuid>,
    pub name: String,
    pub operational_expenses: i64,
    pub headquarter_building_uuid: Uuid,
}
