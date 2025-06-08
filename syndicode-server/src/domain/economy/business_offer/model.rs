use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct BusinessOffer {
    pub uuid: Uuid,
    pub business_uuid: Uuid,
    pub offering_corporation_uuid: Uuid,
    pub target_corporation_uuid: Option<Uuid>,
    pub offer_price: i64,
}
