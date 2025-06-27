use bon::Builder;
use uuid::Uuid;

#[derive(Builder)]
pub struct BuildingOwnership {
    pub building_uuid: Uuid,
    pub owning_business_uuid: Uuid,
}
