use crate::{
    application::error::ApplicationResult,
    domain::economy::business::{model::Business, repository::BusinessRepository},
};
use bon::Builder;
use std::sync::Arc;

#[derive(Builder)]
pub struct ListBusinessesUseCase<BSN>
where
    BSN: BusinessRepository,
{
    business_repo: Arc<BSN>,
}

impl<BSN> ListBusinessesUseCase<BSN>
where
    BSN: BusinessRepository,
{
    pub async fn execute(&self, game_tick: i64) -> ApplicationResult<Vec<Business>> {
        Ok(self
            .business_repo
            .list_businesses_in_tick(game_tick)
            .await?)
    }
}
