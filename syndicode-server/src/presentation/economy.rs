use std::sync::Arc;

use bon::Builder;
use syndicode_proto::{
    syndicode_economy_v1::{
        BuildingOwnershipDetails, BuildingOwnershipsResponse, QueryBuildingOwnershipsRequest,
    },
    syndicode_interface_v1::economy_service_server::EconomyService,
};
use tonic::Response;

use crate::{
    application::{
        economy::query_building_ownerships::QueryBuildingOwnershipsUseCase,
        ports::limiter::{LimiterCategory, RateLimitEnforcer},
    },
    domain::economy::building_ownership::repository::BuildingOwnershipRepository,
};

use super::{
    common::{check_rate_limit, parse_maybe_uuid},
    error::PresentationError,
};

#[derive(Builder)]
pub struct EconomyPresenter<R, BUO>
where
    R: RateLimitEnforcer,
    BUO: BuildingOwnershipRepository,
{
    pub limit: Arc<R>,
    pub query_building_ownerships_uc: Arc<QueryBuildingOwnershipsUseCase<BUO>>,
}

#[tonic::async_trait]
impl<R, BUO> EconomyService for EconomyPresenter<R, BUO>
where
    R: RateLimitEnforcer + 'static,
    BUO: BuildingOwnershipRepository + 'static,
{
    async fn query_building_ownerships(
        &self,
        request: tonic::Request<QueryBuildingOwnershipsRequest>,
    ) -> Result<tonic::Response<BuildingOwnershipsResponse>, tonic::Status> {
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

        let (game_tick, application_ownerships) = self
            .query_building_ownerships_uc
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

        let total_count = application_ownerships.len();
        let mut ownerships = Vec::with_capacity(total_count);

        for o in application_ownerships {
            ownerships.push(BuildingOwnershipDetails { gml_id: o.gml_id });
        }

        Ok(Response::new(BuildingOwnershipsResponse {
            game_tick,
            total_count: total_count as i64,
            ownerships,
        }))
    }
}
