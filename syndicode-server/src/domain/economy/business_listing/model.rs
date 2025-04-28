use bon::Builder;
use uuid::Uuid;

#[derive(Builder, Debug, Clone, Copy, PartialEq)]
pub struct BusinessListing {
    pub uuid: Uuid,
    pub business_uuid: Uuid,
    pub seller_corporation_uuid: Option<Uuid>,
    pub asking_price: i64,
}
