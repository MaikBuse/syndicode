use bon::Builder;
use geo::Point;
use uuid::Uuid;

#[derive(Builder, Clone)]
pub struct Business {
    pub uuid: Uuid,
    pub market_uuid: Uuid,
    pub owning_corporation_uuid: Option<Uuid>,
    pub name: String,
    pub operational_expenses: i64,
    pub center: Point<f64>,
}
