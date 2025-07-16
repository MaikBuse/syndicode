use super::model::Business;
use crate::domain::repository::{DomainSortDirection, RepositoryResult};
use bon::Builder;
use tonic::async_trait;
use uuid::Uuid;

#[derive(Default, Clone, PartialEq)]
pub enum DomainBusinessSortBy {
    #[default]
    Name,
    OperationExpenses,
    MarketVolume,
}

impl Default for &DomainBusinessSortBy {
    fn default() -> Self {
        &DomainBusinessSortBy::Name
    }
}

#[derive(Builder, Clone, PartialEq)]
pub struct QueryBusinessesRequest {
    pub owning_corporation_uuid: Option<Uuid>,
    pub market_uuid: Option<Uuid>,
    pub min_operational_expenses: Option<i64>,
    pub max_operational_expenses: Option<i64>,
    pub sort_by: Option<DomainBusinessSortBy>,
    pub sort_direction: Option<DomainSortDirection>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct BusinessDetails {
    pub business_uuid: Uuid,
    pub business_name: String,
    pub owning_corporation_uuid: Option<Uuid>,
    pub market_uuid: Uuid,
    pub operational_expenses: i64,
    pub headquarter_building_uuid: Uuid,
    pub headquarter_building_gml_id: String,
}

#[async_trait]
pub trait BusinessRepository: Send + Sync {
    async fn query_businesses(
        &self,
        req: &QueryBusinessesRequest,
    ) -> RepositoryResult<(i64, Vec<BusinessDetails>)>;

    async fn list_businesses_in_tick(&self, game_tick: i64) -> RepositoryResult<Vec<Business>>;
}

#[async_trait]
pub trait BusinessTxRepository: Send + Sync {
    async fn insert_businesses_in_tick(
        &mut self,
        game_tick: i64,
        businesses: &[Business],
    ) -> RepositoryResult<()>;

    async fn delete_businesses_before_tick(&mut self, game_tick: i64) -> RepositoryResult<u64>;
}
