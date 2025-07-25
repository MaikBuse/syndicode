use crate::{
    application::{
        economy::{
            acquire_listed_business::AcquireListedBusinessUseCase,
            get_corporation::GetCorporationUseCase,
            query_business_listings::QueryBusinessListingsUseCase,
            query_businesses::QueryBusinessesUseCase,
        },
        game::get_game_tick::GetGameTickUseCase,
        ports::{game_tick::GameTickRepository, queuer::ActionQueueable},
    },
    domain::{
        economy::{
            business::repository::{BusinessRepository, DomainBusinessSortBy},
            business_listing::repository::{
                BusinessListingRepository, DomainBusinessListingSortBy,
            },
            corporation::repository::CorporationRepository,
        },
        repository::DomainSortDirection,
    },
    presentation::{common::parse_maybe_uuid, error::PresentationError},
};
use bon::builder;
use std::sync::Arc;
use syndicode_proto::{
    syndicode_economy_v1::{
        BusinessDetails, BusinessListingDetails, BusinessListingSortBy, BusinessSortBy,
        Corporation, GetCorporationResponse, QueryBusinessListingsRequest,
        QueryBusinessListingsResponse, QueryBusinessesRequest, QueryBusinessesResponse,
    },
    syndicode_interface_v1::{game_update::Update, ActionInitResponse, GameUpdate, SortDirection},
};
use tonic::Status;
use uuid::Uuid;

#[builder]
pub async fn get_corporation<GTR, CRP>(
    get_game_tick_uc: Arc<GetGameTickUseCase<GTR>>,
    get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    user_uuid: Uuid,
    request_uuid: Uuid,
) -> Result<GameUpdate, Status>
where
    GTR: GameTickRepository,
    CRP: CorporationRepository,
{
    match get_corporation_uc.execute(user_uuid).await {
        Ok(outcome) => Ok(GameUpdate {
            game_tick: outcome.game_tick,
            update: Some(Update::GetCorporation(GetCorporationResponse {
                request_uuid: request_uuid.to_string(),
                corporation: Some(Corporation {
                    uuid: outcome.corporation.uuid.to_string(),
                    user_uuid: outcome.corporation.user_uuid.to_string(),
                    name: outcome.corporation.name.to_string(),
                    balance: outcome.corporation.cash_balance,
                }),
            })),
        }),
        Err(err) => {
            let game_tick = get_game_tick_uc.execute().await.unwrap_or_default();

            Ok(PresentationError::from(err).into_game_update(game_tick, request_uuid.to_string()))
        }
    }
}

#[builder]
pub async fn acquire_listed_business<Q, GTR>(
    get_game_tick_uc: Arc<GetGameTickUseCase<GTR>>,
    acquire_listed_business_uc: Arc<AcquireListedBusinessUseCase<Q, GTR>>,
    request_uuid: Uuid,
    req_user_uuid: Uuid,
    business_listing_uuid: String,
) -> Result<GameUpdate, Status>
where
    Q: ActionQueueable,
    GTR: GameTickRepository,
{
    let Ok(business_listing_uuid) = Uuid::parse_str(&business_listing_uuid) else {
        let game_tick = get_game_tick_uc.execute().await.unwrap_or_default();

        let game_update =
            PresentationError::InvalidArgument("Invalid business listing UUID".to_string())
                .into_game_update(game_tick, request_uuid.to_string());

        return Ok(game_update);
    };

    match acquire_listed_business_uc
        .execute()
        .req_user_uuid(req_user_uuid)
        .request_uuid(request_uuid)
        .business_listing_uuid(business_listing_uuid)
        .call()
        .await
    {
        Ok(game_tick) => Ok(GameUpdate {
            game_tick,
            update: Some(Update::ActionInitResponse(ActionInitResponse {
                request_uuid: request_uuid.to_string(),
            })),
        }),
        Err(err) => {
            let game_tick = get_game_tick_uc.execute().await.unwrap_or_default();

            Ok(PresentationError::from(err).into_game_update(game_tick, request_uuid.to_string()))
        }
    }
}

#[builder]
pub async fn query_business_listings<GTR, BL>(
    get_game_tick_uc: Arc<GetGameTickUseCase<GTR>>,
    query_business_listings_uc: Arc<QueryBusinessListingsUseCase<BL>>,
    req: QueryBusinessListingsRequest,
    request_uuid: Uuid,
) -> Result<GameUpdate, Status>
where
    GTR: GameTickRepository,
    BL: BusinessListingRepository,
{
    let seller_corporation_uuid =
        parse_maybe_uuid(req.seller_corporation_uuid, "seller corporation uuid")
            .map_err(|status| *status)?;

    let market_uuid = parse_maybe_uuid(req.market_uuid, "market uuid").map_err(|status| *status)?;

    let sort_by = BusinessListingSortBy::try_from(req.sort_by)
        .map_err(|err| Status::invalid_argument(format!("Failed to parse sort by: {err}")))?;

    let maybe_sort_by = match sort_by {
        BusinessListingSortBy::SortByUnspecified => None,
        BusinessListingSortBy::Price => Some(DomainBusinessListingSortBy::Price),
        BusinessListingSortBy::Name => Some(DomainBusinessListingSortBy::Name),
        BusinessListingSortBy::OperationExpenses => {
            Some(DomainBusinessListingSortBy::OperationExpenses)
        }
        BusinessListingSortBy::MarketVolume => Some(DomainBusinessListingSortBy::MarketVolume),
    };

    let sort_direction = SortDirection::try_from(req.sort_direction).map_err(|err| {
        Status::invalid_argument(format!("Failed to parse sort direction: {err}"))
    })?;

    let maybe_domain_sort_direction = match sort_direction {
        SortDirection::Unspecified => None,
        SortDirection::Ascending => Some(DomainSortDirection::Ascending),
        SortDirection::Descending => Some(DomainSortDirection::Descending),
    };

    match query_business_listings_uc
        .execute()
        .maybe_market_uuid(market_uuid)
        .maybe_seller_corporation_uuid(seller_corporation_uuid)
        .maybe_min_operational_expenses(req.min_operational_expenses)
        .maybe_max_operational_expenses(req.max_operational_expenses)
        .maybe_min_asking_price(req.min_asking_price)
        .maybe_max_asking_price(req.max_asking_price)
        .maybe_sort_by(maybe_sort_by)
        .maybe_sort_direction(maybe_domain_sort_direction)
        .maybe_limit(req.limit)
        .maybe_offset(req.offset)
        .call()
        .await
    {
        Ok((game_tick, listing_details)) => {
            let count = listing_details.len();
            let mut listings = Vec::with_capacity(count);

            for l in listing_details {
                let listing = BusinessListingDetails {
                    listing_uuid: l.listing_uuid.to_string(),
                    business_uuid: l.business_uuid.to_string(),
                    business_name: l.business_name.to_string(),
                    seller_corporation_uuid: l.seller_corporation_uuid.map(|s| s.to_string()),
                    market_uuid: l.market_uuid.to_string(),
                    market_name: l.market_name.to_string(),
                    asking_price: l.asking_price,
                    operational_expenses: l.operational_expenses,
                    headquarter_building_gml_id: l.headquarter_building_gml_id,
                    headquarter_longitude: l.headquarter_longitude,
                    headquarter_latitude: l.headquarter_latitude,
                    image_number: l.image_number as i32,
                    market_number: l.market_number as i32,
                };

                listings.push(listing);
            }

            Ok(GameUpdate {
                game_tick,
                update: Some(Update::QueryBusinessListings(
                    QueryBusinessListingsResponse {
                        request_uuid: request_uuid.to_string(),
                        listings,
                        total_count: count as i64,
                    },
                )),
            })
        }
        Err(err) => {
            let game_tick = get_game_tick_uc.execute().await.unwrap_or_default();

            Ok(PresentationError::from(err).into_game_update(game_tick, request_uuid.to_string()))
        }
    }
}

#[builder]
pub async fn query_businesses<GTR, B>(
    get_game_tick_uc: Arc<GetGameTickUseCase<GTR>>,
    query_businesses_uc: Arc<QueryBusinessesUseCase<B>>,
    req: QueryBusinessesRequest,
    request_uuid: Uuid,
) -> Result<GameUpdate, Status>
where
    GTR: GameTickRepository,
    B: BusinessRepository,
{
    let owning_corporation_uuid =
        parse_maybe_uuid(req.owning_corporation_uuid, "owning corporation uuid")
            .map_err(|status| *status)?;

    let market_uuid = parse_maybe_uuid(req.market_uuid, "market uuid").map_err(|status| *status)?;

    let sort_by = BusinessSortBy::try_from(req.sort_by)
        .map_err(|err| Status::invalid_argument(format!("Failed to parse sort by: {err}")))?;

    let maybe_sort_by = match sort_by {
        BusinessSortBy::Unspecified => None,
        BusinessSortBy::BusinessName => Some(DomainBusinessSortBy::Name),
        BusinessSortBy::BusinessOperationExpenses => Some(DomainBusinessSortBy::OperationExpenses),
        BusinessSortBy::BusinessMarketVolume => Some(DomainBusinessSortBy::MarketVolume),
    };

    let sort_direction = SortDirection::try_from(req.sort_direction).map_err(|err| {
        Status::invalid_argument(format!("Failed to parse sort direction: {err}"))
    })?;

    let maybe_domain_sort_direction = match sort_direction {
        SortDirection::Unspecified => None,
        SortDirection::Ascending => Some(DomainSortDirection::Ascending),
        SortDirection::Descending => Some(DomainSortDirection::Descending),
    };

    match query_businesses_uc
        .execute()
        .maybe_owning_corporation_uuid(owning_corporation_uuid)
        .maybe_market_uuid(market_uuid)
        .maybe_min_operational_expenses(req.min_operational_expenses)
        .maybe_max_operational_expenses(req.max_operational_expenses)
        .maybe_sort_by(maybe_sort_by)
        .maybe_sort_direction(maybe_domain_sort_direction)
        .maybe_limit(req.limit)
        .maybe_offset(req.offset)
        .call()
        .await
    {
        Ok((game_tick, business_details)) => {
            let count = business_details.len();
            let mut businesses = Vec::with_capacity(count);

            for b in business_details {
                let business = BusinessDetails {
                    business_uuid: b.business_uuid.to_string(),
                    business_name: b.business_name.to_string(),
                    owning_corporation_uuid: b.owning_corporation_uuid.map(|s| s.to_string()),
                    market_uuid: b.market_uuid.to_string(),
                    market_name: b.market_name.to_string(),
                    operational_expenses: b.operational_expenses,
                    headquarter_building_uuid: b.headquarter_building_uuid.to_string(),
                    headquarter_building_gml_id: b.headquarter_building_gml_id.to_string(),
                    headquarter_longitude: b.headquarter_longitude,
                    headquarter_latitude: b.headquarter_latitude,
                    image_number: b.image_number as i32,
                    market_number: b.market_number as i32,
                };

                businesses.push(business);
            }

            Ok(GameUpdate {
                game_tick,
                update: Some(Update::QueryBusinesses(QueryBusinessesResponse {
                    request_uuid: request_uuid.to_string(),
                    businesses,
                    total_count: count as i64,
                })),
            })
        }
        Err(err) => {
            let game_tick = get_game_tick_uc.execute().await.unwrap_or_default();

            Ok(PresentationError::from(err).into_game_update(game_tick, request_uuid.to_string()))
        }
    }
}
