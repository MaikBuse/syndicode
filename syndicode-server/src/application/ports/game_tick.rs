use crate::domain::repository::RepositoryResult;

#[tonic::async_trait]
pub trait GameTickRepository: Send + Sync {
    async fn get_current_game_tick(&self) -> RepositoryResult<i64>;
    async fn update_current_game_tick(&self, new_game_tick: i64) -> RepositoryResult<()>;
}

#[tonic::async_trait]
pub trait GameTickTxRepository: Send + Sync {
    async fn get_current_game_tick(&mut self) -> RepositoryResult<i64>;
    async fn update_current_game_tick(&mut self, new_game_tick: i64) -> RepositoryResult<()>;
}
