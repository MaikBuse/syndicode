use bon::Builder;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::domain::game::GameRepository;

#[derive(Builder, Debug)]
pub struct QueryBusinessListingsUseCase<GAME>
where
    GAME: GameRepository,
{
    game_repo: Arc<Mutex<GAME>>,
}

impl<GAME> QueryBusinessListingsUseCase<GAME>
where
    GAME: GameRepository,
{
    pub async fn execute(&mut self) -> anyhow::Result<()> {
        self.game_repo.lock().await.query_business_listings().await
    }
}
