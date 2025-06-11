use crate::{
    application::{
        economy::{
            acquire_listed_business::AcquireListedBusinessUseCase,
            get_corporation::GetCorporationUseCase,
            query_business_listings::QueryBusinessListingsUseCase,
        },
        game::get_game_tick::GetGameTickUseCase,
        ports::{game_tick::GameTickRepository, queuer::ActionQueueable},
    },
    domain::economy::{
        business_listing::repository::BusinessListingRepository,
        corporation::repository::CorporationRepository,
    },
    presentation::{common::parse_maybe_uuid, error::PresentationError},
};
use bon::builder;
use std::sync::Arc;
use syndicode_proto::{
    syndicode_economy_v1::{
        BusinessListingDetails, Corporation, GetCorporationResponse, QueryBusinessListingsRequest,
        QueryBusinessListingsResponse,
    },
    syndicode_interface_v1::{game_update::Update, ActionInitResponse, GameUpdate},
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

    match query_business_listings_uc
        .execute()
        .maybe_min_operational_expenses(req.min_operational_expenses)
        .maybe_max_operational_expenses(req.max_operational_expenses)
        .maybe_min_asking_price(req.min_asking_price)
        .maybe_max_asking_price(req.max_asking_price)
        .maybe_seller_corporation_uuid(seller_corporation_uuid)
        .call()
        .await
    {
        Ok((game_tick, result)) => {
            let mut listings = Vec::with_capacity(result.listings.len());

            for r in result.listings {
                let listing = BusinessListingDetails {
                    listing_uuid: r.listing_uuid.to_string(),
                    business_uuid: r.business_uuid.to_string(),
                    business_name: r.business_name.to_string(),
                    seller_corporation_uuid: r.seller_corporation_uuid.map(|s| s.to_string()),
                    market_uuid: r.market_uuid.to_string(),
                    asking_price: r.asking_price,
                    operational_expenses: r.operational_expenses,
                };

                listings.push(listing);
            }

            Ok(GameUpdate {
                game_tick,
                update: Some(Update::QueryBusinessListings(
                    QueryBusinessListingsResponse {
                        request_uuid: request_uuid.to_string(),
                        listings,
                        total_count: result.total_count,
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
