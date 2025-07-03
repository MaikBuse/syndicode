use std::sync::Arc;

use bon::Builder;
use syndicode_proto::{
    syndicode_economy_v1::{BuildingDetails, QueryBuildingsRequest, QueryBuildingsResponse},
    syndicode_interface_v1::economy_service_server::EconomyService,
};
use tonic::Response;

use crate::{
    application::{
        economy::query_buildings::QueryBuildingsUseCase,
        ports::limiter::{LimiterCategory, RateLimitEnforcer},
    },
    domain::economy::building::repository::BuildingRepository,
};

use super::{
    common::{check_rate_limit, parse_maybe_uuid},
    error::PresentationError,
};

#[derive(Builder)]
pub struct EconomyPresenter<R, BUI>
where
    R: RateLimitEnforcer,
    BUI: BuildingRepository,
{
    pub limit: Arc<R>,
    pub query_buildings_uc: Arc<QueryBuildingsUseCase<BUI>>,
}

#[tonic::async_trait]
impl<R, BUI> EconomyService for EconomyPresenter<R, BUI>
where
    R: RateLimitEnforcer + 'static,
    BUI: BuildingRepository + 'static,
{
    async fn query_buildings(
        &self,
        request: tonic::Request<QueryBuildingsRequest>,
    ) -> Result<tonic::Response<QueryBuildingsResponse>, tonic::Status> {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            LimiterCategory::Game,
        )
        .await
        .map_err(|status| *status)?;

        let request = request.into_inner();

        let owning_corporation_uuid =
            parse_maybe_uuid(request.owning_corporation_uuid, "owning corporation uuid")
                .map_err(|status| *status)?;

        let (game_tick, domain_buildings) = self
            .query_buildings_uc
            .execute()
            .maybe_owning_corporation_uuid(owning_corporation_uuid)
            .maybe_min_lon(request.min_lon)
            .maybe_max_lon(request.max_lon)
            .maybe_min_lat(request.min_lat)
            .maybe_max_lat(request.max_lat)
            .maybe_limit(request.limit)
            .call()
            .await
            .map_err(PresentationError::from)?;

        let total_count = domain_buildings.len();
        let mut buildings = Vec::with_capacity(total_count);

        for o in domain_buildings {
            buildings.push(BuildingDetails { gml_id: o.gml_id });
        }

        Ok(Response::new(QueryBuildingsResponse {
            game_tick,
            total_count: total_count as i64,
            buildings,
        }))
    }
}
