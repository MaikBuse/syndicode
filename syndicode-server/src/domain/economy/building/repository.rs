use tonic::async_trait;

use crate::domain::repository::RepositoryResult;

use super::model::Building;

#[async_trait]
pub trait BuildingTxRepository: Send + Sync {
    async fn insert_buildings(&mut self, buildings: Vec<Building>) -> RepositoryResult<()>;
}
