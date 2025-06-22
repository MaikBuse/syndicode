use tonic::async_trait;

use crate::domain::repository::RepositoryResult;

use super::model::Building;

#[async_trait]
pub trait BuildingRepository: Send + Sync {
    async fn list_buildings_in_tick(&self, game_tick: i64) -> RepositoryResult<Vec<Building>>;
}

#[async_trait]
pub trait BuildingTxRepository: Send + Sync {
    async fn insert_buildings_in_tick(
        &mut self,
        game_tick: i64,
        buildings: Vec<Building>,
    ) -> RepositoryResult<()>;

    async fn delete_buildings_before_tick(&mut self, game_tick: i64) -> RepositoryResult<u64>;
}
