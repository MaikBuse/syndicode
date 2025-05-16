use crate::domain::game::GameRepository;
use bon::Builder;
use std::sync::Arc;
use syndicode_proto::syndicode_interface_v1::GameUpdate;
use tokio::sync::Mutex;
use tonic::Streaming;

#[derive(Builder, Debug)]
pub struct PlayStreamUseCase<GAME>
where
    GAME: GameRepository,
{
    game_repo: Arc<Mutex<GAME>>,
}

impl<GAME> PlayStreamUseCase<GAME>
where
    GAME: GameRepository,
{
    pub async fn execute(&mut self, token: String) -> anyhow::Result<Streaming<GameUpdate>> {
        self.game_repo.lock().await.play_stream(token).await
    }
}
