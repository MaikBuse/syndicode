use super::model::Business;
use crate::domain::repository::RepositoryResult;
use tonic::async_trait;

#[async_trait]
pub trait BusinessTxRepository: Send + Sync {
    async fn insert_businesses_in_tick(
        &mut self,
        game_tick: i64,
        businesses: Vec<Business>,
    ) -> RepositoryResult<()>;
}
