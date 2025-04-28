use crate::{
    application::error::ApplicationResult,
    domain::economy::corporation::{model::Corporation, repository::CorporationRepository},
};
use bon::Builder;
use std::sync::Arc;

#[derive(Builder)]
pub struct ListCorporationsUseCase<CRP>
where
    CRP: CorporationRepository,
{
    corporation_repo: Arc<CRP>,
}

impl<CRP> ListCorporationsUseCase<CRP>
where
    CRP: CorporationRepository,
{
    pub async fn execute(&self, game_tick: i64) -> ApplicationResult<Vec<Corporation>> {
        let corporations = self
            .corporation_repo
            .list_corporations_in_tick(game_tick)
            .await?;

        Ok(corporations)
    }
}
