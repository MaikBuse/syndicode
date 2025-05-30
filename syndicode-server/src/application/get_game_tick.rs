use std::sync::Arc;

use bon::Builder;

use crate::application::error::ApplicationResult;

use super::ports::game_tick::GameTickRepository;

#[derive(Builder)]
pub struct GetGameTickUseCase<GTR>
where
    GTR: GameTickRepository,
{
    game_tick_repo: Arc<GTR>,
}

impl<GTR> GetGameTickUseCase<GTR>
where
    GTR: GameTickRepository,
{
    pub async fn execute(&self) -> ApplicationResult<i64> {
        Ok(self.game_tick_repo.get_current_game_tick().await?)
    }
}
