use crate::{
    application::error::ApplicationResult,
    domain::{
        economy::business::repository::{
            BusinessDetails, BusinessRepository, DomainBusinessSortBy, QueryBusinessesRequest,
        },
        repository::DomainSortDirection,
    },
};
use bon::{bon, Builder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct QueryBusinessesUseCase<B>
where
    B: BusinessRepository,
{
    business_repo: Arc<B>,
}

#[bon]
impl<B> QueryBusinessesUseCase<B>
where
    B: BusinessRepository,
{
    #[builder]
    pub async fn execute(
        &self,
        owning_corporation_uuid: Option<Uuid>,
        market_uuid: Option<Uuid>,
        min_operational_expenses: Option<i64>,
        max_operational_expenses: Option<i64>,
        sort_by: Option<DomainBusinessSortBy>,
        sort_direction: Option<DomainSortDirection>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> ApplicationResult<(i64, Vec<BusinessDetails>)> {
        let req = QueryBusinessesRequest::builder()
            .maybe_owning_corporation_uuid(owning_corporation_uuid)
            .maybe_market_uuid(market_uuid)
            .maybe_min_operational_expenses(min_operational_expenses)
            .maybe_max_operational_expenses(max_operational_expenses)
            .maybe_sort_by(sort_by)
            .maybe_sort_direction(sort_direction)
            .maybe_limit(limit)
            .maybe_offset(offset)
            .build();

        Ok(self.business_repo.query_businesses(&req).await?)
    }
}