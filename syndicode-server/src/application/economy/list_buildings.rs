use crate::{
    application::error::ApplicationResult,
    domain::economy::building::{model::Building, repository::BuildingRepository},
};
use bon::Builder;
use std::sync::Arc;

#[derive(Builder)]
pub struct ListBuildingsUseCase<BLD>
where
    BLD: BuildingRepository,
{
    building_repo: Arc<BLD>,
}

impl<BLD> ListBuildingsUseCase<BLD>
where
    BLD: BuildingRepository,
{
    pub async fn execute(&self, game_tick: i64) -> ApplicationResult<Vec<Building>> {
        Ok(self.building_repo.list_buildings_in_tick(game_tick).await?)
    }
}
