pub mod name;

use bon::Builder;
use name::MarketName;
use uuid::Uuid;

#[derive(Builder)]
pub struct Market {
    pub uuid: Uuid,
    pub name: MarketName,
    pub volume: i64,
}
