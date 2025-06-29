use bon::Builder;
use tonic::async_trait;
use uuid::Uuid;

use crate::domain::repository::RepositoryResult;

use super::model::BuildingOwnership;

#[derive(Builder, Clone, PartialEq)]
pub struct QueryBuildingOwnershipsRequest {
    pub owning_corporation_uuid: Option<Uuid>,
    pub min_lon: Option<f64>,
    pub max_lon: Option<f64>,
    pub min_lat: Option<f64>,
    pub max_lat: Option<f64>,
    pub limit: Option<i64>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct BuildingOwnershipDetails {
    pub gml_id: String,
}

#[async_trait]
pub trait BuildingOwnershipRepository: Send + Sync {
    async fn list_building_ownerships_in_tick(
        &self,
        game_tick: i64,
    ) -> RepositoryResult<Vec<BuildingOwnership>>;

    async fn query_building_ownerships(
        &self,
        req: QueryBuildingOwnershipsRequest,
    ) -> RepositoryResult<(i64, Vec<BuildingOwnershipDetails>)>;
}

#[async_trait]
pub trait BuildingOwnershipTxRepository: Send + Sync {
    async fn insert_building_ownerships_in_tick(
        &mut self,
        game_tick: i64,
        building_ownerships: &[BuildingOwnership],
    ) -> RepositoryResult<()>;

    async fn delete_building_ownerships_before_tick(
        &mut self,
        game_tick: i64,
    ) -> RepositoryResult<u64>;
}
