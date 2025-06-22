use bon::Builder;
use geo::{Point, Polygon};
use uuid::Uuid;

#[derive(Builder, Clone)]
pub struct Building {
    pub uuid: Uuid,
    pub gml_id: String,
    pub name: String,
    pub owning_business_uuid: Option<Uuid>,
    pub address: String,
    pub usage: String,
    pub usage_code: i16,
    pub class: String,
    pub class_code: i16,
    pub city: String,
    pub city_code: String,
    /// The calculated centroid of the building's footprint
    pub center: Point<f64>,
    pub footprint: Polygon,
    pub height: f64,
    pub prefecture: String,
}
