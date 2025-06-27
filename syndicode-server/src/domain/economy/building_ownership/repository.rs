use tonic::async_trait;

use crate::domain::repository::RepositoryResult;

use super::model::BuildingOwnership;

#[async_trait]
pub trait BuildingOwnershipRepository: Send + Sync {
    async fn list_building_ownerships_in_tick(
        &self,
        game_tick: i64,
    ) -> RepositoryResult<Vec<BuildingOwnership>>;
}

#[async_trait]
pub trait BuildingOwnershipTxRepository: Send + Sync {
    async fn insert_building_ownerships_in_tick(
        &mut self,
        game_tick: i64,
        building_ownerships: Vec<BuildingOwnership>,
    ) -> RepositoryResult<()>;

    async fn delete_building_ownerships_before_tick(
        &mut self,
        game_tick: i64,
    ) -> RepositoryResult<u64>;
}
