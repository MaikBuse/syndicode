use bon::{bon, Builder};
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

#[bon]
impl<GAME> QueryBusinessListingsUseCase<GAME>
where
    GAME: GameRepository,
{
    #[builder]
    pub async fn execute(
        &mut self,
        min_asking_price: Option<i64>,
        max_asking_price: Option<i64>,
        seller_corporation_uuid: Option<String>,
        market_uuid: Option<String>,
        min_operational_expenses: Option<i64>,
        max_operational_expenses: Option<i64>,
        sort_by: String,
        sort_direction: i32,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> anyhow::Result<()> {
        self.game_repo
            .lock()
            .await
            .query_business_listings(
                min_asking_price,
                max_asking_price,
                seller_corporation_uuid,
                market_uuid,
                min_operational_expenses,
                max_operational_expenses,
                sort_by,
                sort_direction,
                limit,
                offset,
            )
            .await
    }
}
