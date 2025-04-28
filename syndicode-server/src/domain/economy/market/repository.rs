use tonic::async_trait;

use super::model::Market;
use crate::domain::repository::RepositoryResult;

#[async_trait]
pub trait MarketRepository: Send + Sync {
    async fn list_markets_in_tick(&self, game_tick: i64) -> RepositoryResult<Vec<Market>>;
}

#[async_trait]
pub trait MarketTxRepository: Send + Sync {
    async fn insert_markets_in_tick(
        &mut self,
        game_tick: i64,
        markets: Vec<Market>,
    ) -> RepositoryResult<()>;
    async fn delete_markets_before_tick(&mut self, game_tick: i64) -> RepositoryResult<u64>;
}
