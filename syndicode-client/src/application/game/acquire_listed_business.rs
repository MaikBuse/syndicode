use bon::{bon, Builder};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::domain::game::GameRepository;

#[derive(Builder, Debug)]
pub struct AcquireListedBusinessUseCase<GAME>
where
    GAME: GameRepository,
{
    game_repo: Arc<Mutex<GAME>>,
}

#[bon]
impl<GAME> AcquireListedBusinessUseCase<GAME>
where
    GAME: GameRepository,
{
    #[builder]
    pub async fn execute(&mut self, business_listing_uuid: String) -> anyhow::Result<()> {
        self.game_repo
            .lock()
            .await
            .acquire_listed_business(business_listing_uuid)
            .await
    }
}
