use bon::Builder;
use uuid::Uuid;

#[derive(Builder)]
pub struct Business {
    pub uuid: Uuid,
    pub market_uuid: Uuid,
    pub owning_corporation_uuid: Option<Uuid>,
    pub name: String,
    pub operational_expenses: i64,
}
