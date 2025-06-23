use bon::Builder;
use geo::{Point, Polygon};
use uuid::Uuid;

#[derive(Builder, Clone)]
pub struct Building {
    pub uuid: Uuid,
    pub gml_id: String,
    pub name: Option<String>,
    pub owning_business_uuid: Option<Uuid>,
    pub address: Option<String>,
    pub usage: Option<String>,
    pub usage_code: Option<String>,
    pub class: Option<String>,
    pub class_code: Option<String>,
    pub city: Option<String>,
    pub city_code: Option<String>,
    /// The calculated centroid of the building's footprint
    pub center: Point<f64>,
    pub footprint: Polygon,
    pub height: f64,
    pub prefecture: Option<String>,
}
