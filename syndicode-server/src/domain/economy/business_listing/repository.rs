use super::model::BusinessListing;
use crate::domain::repository::{DomainSortDirection, RepositoryResult};
use bon::Builder;
use tonic::async_trait;
use uuid::Uuid;

#[derive(Default, Clone, PartialEq)]
pub enum DomainBusinessListingSortBy {
    #[default]
    Price,
    Name,
    OperationExpenses,
    MarketVolume,
}

impl Default for &DomainBusinessListingSortBy {
    fn default() -> Self {
        &DomainBusinessListingSortBy::Price
    }
}

#[derive(Builder, Clone, PartialEq)]
pub struct QueryBusinessListingsRequest {
    pub market_uuid: Option<Uuid>,
    pub min_asking_price: Option<i64>,
    pub max_asking_price: Option<i64>,
    pub seller_corporation_uuid: Option<Uuid>,
    pub min_operational_expenses: Option<i64>,
    pub max_operational_expenses: Option<i64>,
    pub sort_by: Option<DomainBusinessListingSortBy>,
    pub sort_direction: Option<DomainSortDirection>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct BusinessListingDetails {
    pub listing_uuid: Uuid,
    pub market_uuid: Uuid,
    pub business_uuid: Uuid,
    pub business_name: String,
    pub seller_corporation_uuid: Option<Uuid>,
    pub asking_price: i64,
    pub operational_expenses: i64,
}

#[async_trait]
pub trait BusinessListingRepository: Send + Sync {
    async fn query_business_listings(
        &self,
        req: &QueryBusinessListingsRequest,
    ) -> RepositoryResult<(i64, Vec<BusinessListingDetails>)>;

    async fn list_business_listings_in_tick(
        &self,
        game_tick: i64,
    ) -> RepositoryResult<Vec<BusinessListing>>;
}

#[async_trait]
pub trait BusinessListingTxRepository: Send + Sync {
    async fn insert_business_listings_in_tick(
        &mut self,
        game_tick: i64,
        business_listings: &[BusinessListing],
    ) -> RepositoryResult<()>;

    async fn delete_business_listings_before_tick(
        &mut self,
        game_tick: i64,
    ) -> RepositoryResult<u64>;
}
