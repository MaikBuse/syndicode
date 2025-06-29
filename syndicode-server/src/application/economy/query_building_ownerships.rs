use crate::{
    application::error::ApplicationResult,
    domain::economy::building_ownership::repository::{
        BuildingOwnershipDetails, BuildingOwnershipRepository, QueryBuildingOwnershipsRequest,
    },
};
use bon::{bon, Builder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct QueryBuildingOwnershipsUseCase<BUO>
where
    BUO: BuildingOwnershipRepository,
{
    building_ownership_repo: Arc<BUO>,
}

#[bon]
impl<BUO> QueryBuildingOwnershipsUseCase<BUO>
where
    BUO: BuildingOwnershipRepository,
{
    #[builder]
    pub async fn execute(
        &self,
        owning_corporation_uuid: Option<Uuid>,
        min_lon: Option<f64>,
        max_lon: Option<f64>,
        min_lat: Option<f64>,
        max_lat: Option<f64>,
        limit: Option<i64>,
    ) -> ApplicationResult<(i64, Vec<BuildingOwnershipDetails>)> {
        let req = QueryBuildingOwnershipsRequest::builder()
            .maybe_owning_corporation_uuid(owning_corporation_uuid)
            .maybe_min_lon(min_lon)
            .maybe_max_lon(max_lon)
            .maybe_min_lat(min_lat)
            .maybe_max_lat(max_lat)
            .maybe_limit(limit)
            .build();

        Ok(self
            .building_ownership_repo
            .query_building_ownerships(req)
            .await?)
    }
}
