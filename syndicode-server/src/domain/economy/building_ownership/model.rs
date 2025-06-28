use bon::Builder;
use uuid::Uuid;

#[derive(Builder, Clone, Copy)]
pub struct BuildingOwnership {
    pub building_uuid: Uuid,
    pub owning_business_uuid: Uuid,
}
