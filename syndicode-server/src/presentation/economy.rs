use std::sync::Arc;

use bon::Builder;
use syndicode_proto::{
    syndicode_economy_v1::{
        BuildingDetails, BusinessDetails, BusinessListingDetails, BusinessListingSortBy, BusinessSortBy, GetCorporationRequest,
        QueryBuildingsRequest, QueryBuildingsResponse, QueryBusinessesRequest,
        QueryBusinessesResponse, QueryBusinessListingsRequest, QueryBusinessListingsResponse,
    },
    syndicode_interface_v1::{economy_service_server::EconomyService, SortDirection},
};
use tonic::Response;

use crate::{
    application::{
        economy::{
            get_corporation::GetCorporationUseCase, query_buildings::QueryBuildingsUseCase,
            query_businesses::QueryBusinessesUseCase, query_business_listings::QueryBusinessListingsUseCase,
        },
        ports::limiter::{LimiterCategory, RateLimitEnforcer},
    },
    domain::{
        economy::{
            building::repository::BuildingRepository,
            business::repository::{BusinessRepository, DomainBusinessSortBy},
            business_listing::repository::{BusinessListingRepository, DomainBusinessListingSortBy},
            corporation::repository::CorporationRepository,
        },
        repository::DomainSortDirection,
    },
};

use super::{
    common::{check_rate_limit, parse_maybe_uuid, uuid_from_metadata},
    error::PresentationError,
};

#[derive(Builder)]
pub struct EconomyPresenter<R, BUI, CRP, B, BL>
where
    R: RateLimitEnforcer,
    BUI: BuildingRepository,
    CRP: CorporationRepository,
    B: BusinessRepository,
    BL: BusinessListingRepository,
{
    pub limit: Arc<R>,
    pub query_buildings_uc: Arc<QueryBuildingsUseCase<BUI>>,
    pub get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    pub query_businesses_uc: Arc<QueryBusinessesUseCase<B>>,
    pub query_business_listings_uc: Arc<QueryBusinessListingsUseCase<BL>>,
}

#[tonic::async_trait]
impl<R, BUI, CRP, B, BL> EconomyService for EconomyPresenter<R, BUI, CRP, B, BL>
where
    R: RateLimitEnforcer + 'static,
    BUI: BuildingRepository + 'static,
    CRP: CorporationRepository + 'static,
    B: BusinessRepository + 'static,
    BL: BusinessListingRepository + 'static,
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

        let owning_business_uuid =
            parse_maybe_uuid(request.owning_business_uuid, "owning business uuid")
                .map_err(|status| *status)?;

        let (game_tick, domain_buildings) = self
            .query_buildings_uc
            .execute()
            .maybe_owning_corporation_uuid(owning_corporation_uuid)
            .maybe_owning_business_uuid(owning_business_uuid)
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
            buildings.push(BuildingDetails { 
                gml_id: o.gml_id,
                longitude: o.longitude,
                latitude: o.latitude,
            });
        }

        Ok(Response::new(QueryBuildingsResponse {
            game_tick,
            total_count: total_count as i64,
            buildings,
        }))
    }

    async fn get_current_corporation(
        &self,
        request: tonic::Request<GetCorporationRequest>,
    ) -> std::result::Result<
        tonic::Response<syndicode_proto::syndicode_economy_v1::Corporation>,
        tonic::Status,
    > {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            LimiterCategory::Game,
        )
        .await
        .map_err(|status| *status)?;

        let req_user_uuid = match uuid_from_metadata(request.metadata()) {
            Ok(uuid) => uuid,
            Err(status) => return Err(*status),
        };

        let outcome = self
            .get_corporation_uc
            .execute(req_user_uuid)
            .await
            .map_err(PresentationError::from)?;

        let corporation = syndicode_proto::syndicode_economy_v1::Corporation {
            uuid: outcome.corporation.uuid.to_string(),
            user_uuid: outcome.corporation.user_uuid.to_string(),
            name: outcome.corporation.name.to_string(),
            balance: outcome.corporation.cash_balance,
        };

        Ok(Response::new(corporation))
    }

    async fn query_businesses(
        &self,
        request: tonic::Request<QueryBusinessesRequest>,
    ) -> Result<tonic::Response<QueryBusinessesResponse>, tonic::Status> {
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

        let market_uuid =
            parse_maybe_uuid(request.market_uuid, "market uuid").map_err(|status| *status)?;

        let sort_by = BusinessSortBy::try_from(request.sort_by).map_err(|err| {
            tonic::Status::invalid_argument(format!("Failed to parse sort by: {err}"))
        })?;

        let maybe_sort_by = match sort_by {
            BusinessSortBy::Unspecified => None,
            BusinessSortBy::BusinessName => Some(DomainBusinessSortBy::Name),
            BusinessSortBy::BusinessOperationExpenses => {
                Some(DomainBusinessSortBy::OperationExpenses)
            }
            BusinessSortBy::BusinessMarketVolume => Some(DomainBusinessSortBy::MarketVolume),
        };

        let sort_direction = SortDirection::try_from(request.sort_direction).map_err(|err| {
            tonic::Status::invalid_argument(format!("Failed to parse sort direction: {err}"))
        })?;

        let maybe_domain_sort_direction = match sort_direction {
            SortDirection::Unspecified => None,
            SortDirection::Ascending => Some(DomainSortDirection::Ascending),
            SortDirection::Descending => Some(DomainSortDirection::Descending),
        };

        let (_, domain_businesses) = self
            .query_businesses_uc
            .execute()
            .maybe_owning_corporation_uuid(owning_corporation_uuid)
            .maybe_market_uuid(market_uuid)
            .maybe_min_operational_expenses(request.min_operational_expenses)
            .maybe_max_operational_expenses(request.max_operational_expenses)
            .maybe_sort_by(maybe_sort_by)
            .maybe_sort_direction(maybe_domain_sort_direction)
            .maybe_limit(request.limit)
            .maybe_offset(request.offset)
            .call()
            .await
            .map_err(PresentationError::from)?;

        let total_count = domain_businesses.len();
        let mut businesses = Vec::with_capacity(total_count);

        for b in domain_businesses {
            businesses.push(BusinessDetails {
                business_uuid: b.business_uuid.to_string(),
                business_name: b.business_name,
                owning_corporation_uuid: b.owning_corporation_uuid.map(|uuid| uuid.to_string()),
                market_uuid: b.market_uuid.to_string(),
                operational_expenses: b.operational_expenses,
                headquarter_building_uuid: b.headquarter_building_uuid.to_string(),
                headquarter_building_gml_id: b.headquarter_building_gml_id,
                headquarter_longitude: b.headquarter_longitude,
                headquarter_latitude: b.headquarter_latitude,
            });
        }

        Ok(Response::new(QueryBusinessesResponse {
            request_uuid: "".to_string(), // Not used in standalone service
            businesses,
            total_count: total_count as i64,
        }))
    }

    async fn query_business_listings(
        &self,
        request: tonic::Request<QueryBusinessListingsRequest>,
    ) -> Result<tonic::Response<QueryBusinessListingsResponse>, tonic::Status> {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            LimiterCategory::Game,
        )
        .await
        .map_err(|status| *status)?;

        let request = request.into_inner();

        let seller_corporation_uuid =
            parse_maybe_uuid(request.seller_corporation_uuid, "seller corporation uuid")
                .map_err(|status| *status)?;

        let market_uuid =
            parse_maybe_uuid(request.market_uuid, "market uuid").map_err(|status| *status)?;

        let sort_by = BusinessListingSortBy::try_from(request.sort_by).map_err(|err| {
            tonic::Status::invalid_argument(format!("Failed to parse sort by: {err}"))
        })?;

        let maybe_sort_by = match sort_by {
            BusinessListingSortBy::SortByUnspecified => None,
            BusinessListingSortBy::Price => Some(DomainBusinessListingSortBy::Price),
            BusinessListingSortBy::Name => Some(DomainBusinessListingSortBy::Name),
            BusinessListingSortBy::OperationExpenses => Some(DomainBusinessListingSortBy::OperationExpenses),
            BusinessListingSortBy::MarketVolume => Some(DomainBusinessListingSortBy::MarketVolume),
        };

        let sort_direction = SortDirection::try_from(request.sort_direction).map_err(|err| {
            tonic::Status::invalid_argument(format!("Failed to parse sort direction: {err}"))
        })?;

        let maybe_domain_sort_direction = match sort_direction {
            SortDirection::Unspecified => None,
            SortDirection::Ascending => Some(DomainSortDirection::Ascending),
            SortDirection::Descending => Some(DomainSortDirection::Descending),
        };

        let (_, domain_listings) = self
            .query_business_listings_uc
            .execute()
            .maybe_market_uuid(market_uuid)
            .maybe_min_asking_price(request.min_asking_price)
            .maybe_max_asking_price(request.max_asking_price)
            .maybe_seller_corporation_uuid(seller_corporation_uuid)
            .maybe_min_operational_expenses(request.min_operational_expenses)
            .maybe_max_operational_expenses(request.max_operational_expenses)
            .maybe_sort_by(maybe_sort_by)
            .maybe_sort_direction(maybe_domain_sort_direction)
            .maybe_limit(request.limit)
            .maybe_offset(request.offset)
            .call()
            .await
            .map_err(PresentationError::from)?;

        let total_count = domain_listings.len();
        let mut listings = Vec::with_capacity(total_count);

        for listing in domain_listings {
            listings.push(BusinessListingDetails {
                listing_uuid: listing.listing_uuid.to_string(),
                business_uuid: listing.business_uuid.to_string(),
                business_name: listing.business_name,
                seller_corporation_uuid: listing.seller_corporation_uuid.map(|uuid| uuid.to_string()),
                market_uuid: listing.market_uuid.to_string(),
                asking_price: listing.asking_price,
                operational_expenses: listing.operational_expenses,
                headquarter_building_gml_id: listing.headquarter_building_gml_id,
                headquarter_longitude: listing.headquarter_longitude,
                headquarter_latitude: listing.headquarter_latitude,
            });
        }

        Ok(Response::new(QueryBusinessListingsResponse {
            request_uuid: "".to_string(), // Not used in standalone service
            listings,
            total_count: total_count as i64,
        }))
    }
}
