use crate::{
    application::error::ApplicationResult,
    domain::economy::business_offer::{model::BusinessOffer, repository::BusinessOfferRepository},
};
use bon::Builder;
use std::sync::Arc;

#[derive(Builder)]
pub struct ListBusinessOffersUseCase<BO>
where
    BO: BusinessOfferRepository,
{
    business_offer_repo: Arc<BO>,
}

impl<BO> ListBusinessOffersUseCase<BO>
where
    BO: BusinessOfferRepository,
{
    pub async fn execute(&self, game_tick: i64) -> ApplicationResult<Vec<BusinessOffer>> {
        Ok(self
            .business_offer_repo
            .list_business_offers_in_tick(game_tick)
            .await?)
    }
}
