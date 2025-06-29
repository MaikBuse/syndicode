use crate::{
    application::error::ApplicationResult,
    domain::{
        economy::business_listing::repository::{
            BusinessListingDetails, BusinessListingRepository, DomainBusinessListingSortBy,
            QueryBusinessListingsRequest,
        },
        repository::DomainSortDirection,
    },
};
use bon::{bon, Builder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct QueryBusinessListingsUseCase<BL>
where
    BL: BusinessListingRepository,
{
    business_listing_repo: Arc<BL>,
}

#[bon]
impl<BL> QueryBusinessListingsUseCase<BL>
where
    BL: BusinessListingRepository,
{
    #[builder]
    pub async fn execute(
        &self,
        market_uuid: Option<Uuid>,
        min_asking_price: Option<i64>,
        max_asking_price: Option<i64>,
        seller_corporation_uuid: Option<Uuid>,
        min_operational_expenses: Option<i64>,
        max_operational_expenses: Option<i64>,
        sort_by: Option<DomainBusinessListingSortBy>,
        sort_direction: Option<DomainSortDirection>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> ApplicationResult<(i64, Vec<BusinessListingDetails>)> {
        let req = QueryBusinessListingsRequest::builder()
            .maybe_market_uuid(market_uuid)
            .maybe_min_asking_price(min_asking_price)
            .maybe_max_asking_price(max_asking_price)
            .maybe_seller_corporation_uuid(seller_corporation_uuid)
            .maybe_min_operational_expenses(min_operational_expenses)
            .maybe_max_operational_expenses(max_operational_expenses)
            .maybe_sort_by(sort_by)
            .maybe_sort_direction(sort_direction)
            .maybe_limit(limit)
            .maybe_offset(offset)
            .build();

        Ok(self
            .business_listing_repo
            .query_business_listings(&req)
            .await?)
    }
}
