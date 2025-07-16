use crate::{
    application::error::ApplicationResult,
    domain::economy::building::repository::{
        BuildingDetails, BuildingRepository, QueryBuildingsRequest,
    },
};
use bon::{bon, Builder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct QueryBuildingsUseCase<BUI>
where
    BUI: BuildingRepository,
{
    building_repo: Arc<BUI>,
}

#[bon]
impl<BUI> QueryBuildingsUseCase<BUI>
where
    BUI: BuildingRepository,
{
    #[builder]
    pub async fn execute(
        &self,
        owning_corporation_uuid: Option<Uuid>,
        owning_business_uuid: Option<Uuid>,
        min_lon: Option<f64>,
        max_lon: Option<f64>,
        min_lat: Option<f64>,
        max_lat: Option<f64>,
        limit: Option<i64>,
    ) -> ApplicationResult<(i64, Vec<BuildingDetails>)> {
        let req = QueryBuildingsRequest::builder()
            .maybe_owning_corporation_uuid(owning_corporation_uuid)
            .maybe_owning_business_uuid(owning_business_uuid)
            .maybe_min_lon(min_lon)
            .maybe_max_lon(max_lon)
            .maybe_min_lat(min_lat)
            .maybe_max_lat(max_lat)
            .maybe_limit(limit)
            .build();

        Ok(self.building_repo.query_buildings(req).await?)
    }
}
