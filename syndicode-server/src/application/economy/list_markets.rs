use std::sync::Arc;

use bon::Builder;

use crate::{
    application::error::ApplicationResult,
    domain::economy::market::{model::Market, repository::MarketRepository},
};

#[derive(Builder)]
pub struct ListMarketsUseCase<MRK>
where
    MRK: MarketRepository,
{
    market_repo: Arc<MRK>,
}

impl<MRK> ListMarketsUseCase<MRK>
where
    MRK: MarketRepository,
{
    pub async fn execute(&self, game_tick: i64) -> ApplicationResult<Vec<Market>> {
        Ok(self.market_repo.list_markets_in_tick(game_tick).await?)
    }
}
