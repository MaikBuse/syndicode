use crate::{
    application::error::ApplicationResult,
    domain::economy::building_ownership::{
        model::BuildingOwnership, repository::BuildingOwnershipRepository,
    },
};
use bon::Builder;
use std::sync::Arc;

#[derive(Builder)]
pub struct ListBuildingOwnershipsUseCase<BLO>
where
    BLO: BuildingOwnershipRepository,
{
    building_ownership_repo: Arc<BLO>,
}

impl<BLO> ListBuildingOwnershipsUseCase<BLO>
where
    BLO: BuildingOwnershipRepository,
{
    pub async fn execute(&self, game_tick: i64) -> ApplicationResult<Vec<BuildingOwnership>> {
        Ok(self
            .building_ownership_repo
            .list_building_ownerships_in_tick(game_tick)
            .await?)
    }
}
