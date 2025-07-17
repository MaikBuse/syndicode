use bon::Builder;
use tonic::async_trait;
use uuid::Uuid;

use crate::domain::repository::RepositoryResult;

use super::model::Building;

#[derive(Builder, Clone, PartialEq)]
pub struct QueryBuildingsRequest {
    pub owning_corporation_uuid: Option<Uuid>,
    pub owning_business_uuid: Option<Uuid>,
    pub min_lon: Option<f64>,
    pub max_lon: Option<f64>,
    pub min_lat: Option<f64>,
    pub max_lat: Option<f64>,
    pub limit: Option<i64>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct BuildingDetails {
    pub gml_id: String,
    pub longitude: f64,
    pub latitude: f64,
}

#[async_trait]
pub trait BuildingRepository: Send + Sync {
    async fn query_buildings(
        &self,
        req: QueryBuildingsRequest,
    ) -> RepositoryResult<(i64, Vec<BuildingDetails>)>;
}

#[async_trait]
pub trait BuildingTxRepository: Send + Sync {
    async fn insert_buildings(&mut self, buildings: Vec<Building>) -> RepositoryResult<()>;
}
