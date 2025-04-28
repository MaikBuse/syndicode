use crate::{
    application::error::ApplicationResult,
    domain::economy::business_listing::{
        model::BusinessListing, repository::BusinessListingRepository,
    },
};
use bon::Builder;
use std::sync::Arc;

#[derive(Builder)]
pub struct ListBusinessListingUseCase<BL>
where
    BL: BusinessListingRepository,
{
    business_listing_repo: Arc<BL>,
}

impl<BL> ListBusinessListingUseCase<BL>
where
    BL: BusinessListingRepository,
{
    pub async fn execute(&self, game_tick: i64) -> ApplicationResult<Vec<BusinessListing>> {
        Ok(self
            .business_listing_repo
            .list_business_listings_in_tick(game_tick)
            .await?)
    }
}
