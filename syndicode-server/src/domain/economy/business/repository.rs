use super::model::Business;
use crate::domain::repository::RepositoryResult;
use tonic::async_trait;

#[async_trait]
pub trait BusinessRepository: Send + Sync {
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
