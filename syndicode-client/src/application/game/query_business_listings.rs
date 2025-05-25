use bon::{bon, Builder};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::domain::game::{GameRepository, QueryBusinessListingsDomainRequest};

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
        let req = QueryBusinessListingsDomainRequest::builder()
            .maybe_min_asking_price(min_asking_price)
            .maybe_max_asking_price(max_asking_price)
            .maybe_seller_corporation_uuid(seller_corporation_uuid)
            .maybe_market_uuid(market_uuid)
            .maybe_min_operational_expenses(min_operational_expenses)
            .maybe_max_operational_expenses(max_operational_expenses)
            .sort_by(sort_by)
            .sort_direction(sort_direction)
            .maybe_limit(limit)
            .maybe_offset(offset)
            .build();

        self.game_repo
            .lock()
            .await
            .query_business_listings(req)
            .await
    }
}
