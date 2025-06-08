mod economy;
pub mod user_channel_guard;
mod warfare;

use super::{
    common::{ip_address_from_metadata, uuid_from_metadata},
    error::PresentationError,
};
use crate::{
    application::{
        economy::{
            acquire_listed_business::AcquireListedBusinessUseCase,
            get_corporation::GetCorporationUseCase,
            query_business_listings::QueryBusinessListingsUseCase,
        },
        game::get_game_tick::GetGameTickUseCase,
        ports::{
            game_tick::GameTickRepository,
            limiter::{LimiterCategory, RateLimitEnforcer},
            outcome::OutcomeStoreReader,
            queuer::ActionQueueable,
        },
        warfare::{
            list_units_by_corporation::ListUnitsByCorporationUseCase, spawn_unit::SpawnUnitUseCase,
        },
    },
    config::Config,
    domain::{
        economy::{
            business_listing::repository::BusinessListingRepository,
            corporation::repository::CorporationRepository,
        },
        outcome::DomainActionOutcome,
        unit::repository::UnitRepository,
    },
    infrastructure::valkey::outcome::create_notification_channel,
    presentation::common::limitation_error_into_status,
};
use bon::{builder, Builder};
use economy::{acquire_listed_business, get_corporation, query_business_listings};
use std::{pin::Pin, str::FromStr, sync::Arc};
use syndicode_proto::{
    syndicode_economy_v1::{
        AcquireListedBusinessResponse, Business, CreateCorporationResponse,
        DeleteCorporationResponse,
    },
    syndicode_interface_v1::{
        game_service_server::GameService, game_update::Update, player_action::Action,
        ActionFailedResponse, GameUpdate, PlayerAction,
    },
    syndicode_warfare_v1::{SpawnUnitResponse, Unit},
};
use tokio::sync::mpsc::{self, error::SendError};
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Code, Request, Response, Status, Streaming};
use user_channel_guard::{UserChannelGuard, UserChannels, UserTx};
use uuid::Uuid;
use warfare::{list_units, spawn_unit};

// Tunable: buffer for server-to-client MPSC channels.
const MPSC_CHANNEL_BUFFER_SIZE: usize = 128;

#[derive(Builder)]
pub struct GamePresenter<R, Q, UNT, CRP, OSR, GTR, BL>
where
    R: RateLimitEnforcer,
    Q: ActionQueueable,
    UNT: UnitRepository,
    CRP: CorporationRepository,
    OSR: OutcomeStoreReader,
    GTR: GameTickRepository,
    BL: BusinessListingRepository,
{
    pub config: Arc<Config>,
    pub valkey_client: redis::Client,
    pub limit: Arc<R>,
    pub outcome_store_reader: Arc<OSR>,
    pub user_channels: UserChannels,
    pub get_game_tick_uc: Arc<GetGameTickUseCase<GTR>>,
    pub get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    pub list_units_by_corporation_uc: Arc<ListUnitsByCorporationUseCase<UNT>>,
    pub spawn_unit_uc: Arc<SpawnUnitUseCase<Q, GTR>>,
    pub acquire_listed_business_uc: Arc<AcquireListedBusinessUseCase<Q, GTR>>,
    pub query_business_listings_uc: Arc<QueryBusinessListingsUseCase<BL>>,
}

#[tonic::async_trait]
impl<R, Q, UNT, CRP, OSR, GTR, BL> GameService for GamePresenter<R, Q, UNT, CRP, OSR, GTR, BL>
where
    R: RateLimitEnforcer + 'static,
    Q: ActionQueueable + 'static,
    UNT: UnitRepository + 'static,
    CRP: CorporationRepository + 'static,
    OSR: OutcomeStoreReader + 'static,
    GTR: GameTickRepository + 'static,
    BL: BusinessListingRepository + 'static,
{
    type PlayStreamStream = Pin<Box<dyn Stream<Item = Result<GameUpdate, Status>> + Send>>;

    async fn play_stream(
        &self,
        request: Request<Streaming<PlayerAction>>,
    ) -> Result<Response<Self::PlayStreamStream>, Status> {
        let ip_address = ip_address_from_metadata(
            request.metadata(),
            &self.config.ip_address_header.to_lowercase(),
        )?;

        let user_uuid = uuid_from_metadata(request.metadata())?;

        // Setup Communication Channel
        let (tx_raw, rx) = mpsc::channel(MPSC_CHANNEL_BUFFER_SIZE);
        let user_channel_tx_arc = Arc::new(tx_raw); // Wrap the sender in an Arc
        let response_stream = ReceiverStream::new(rx);

        // Register User Channel
        if self
            .user_channels
            .insert(user_uuid, user_channel_tx_arc.clone())
            .is_some()
        {
            tracing::warn!(
                "User {} connected again, overwriting previous channel entry in map.",
                user_uuid
            );
        }

        // Clone Arcs needed for the spawned tasks.
        let get_game_tick_uc = Arc::clone(&self.get_game_tick_uc);
        let get_corporation_uc = Arc::clone(&self.get_corporation_uc);
        let list_units_by_corporation_uc = Arc::clone(&self.list_units_by_corporation_uc);
        let acquire_listed_business_uc = Arc::clone(&self.acquire_listed_business_uc);
        let query_business_listings_uc = Arc::clone(&self.query_business_listings_uc);
        let spawn_unit_uc = Arc::clone(&self.spawn_unit_uc);

        let limit = Arc::clone(&self.limit);
        let user_channels_clone_for_guard = Arc::clone(&self.user_channels);
        let outcome_reader_clone = Arc::clone(&self.outcome_store_reader);

        let user_channel_tx_arc_for_action_task = user_channel_tx_arc.clone();
        let user_channel_tx_arc_for_outcome_task = user_channel_tx_arc.clone();

        // Task 1: Handle Incoming Client Actions
        tokio::spawn(async move {
            let _channel_guard = UserChannelGuard {
                user_id: user_uuid,
                channels: user_channels_clone_for_guard,
                channel_instance: user_channel_tx_arc.clone(), // Guard is responsible for this specific Arc<UserTx>
            };

            let mut stream = request.into_inner();

            while let Some(stream_result) = stream.next().await {
                match stream_result {
                    Ok(player_action) => {
                        if let Err(err) =
                            limit.check(LimiterCategory::GameStream, &ip_address).await
                        {
                            let status = limitation_error_into_status(err);
                            if (*user_channel_tx_arc_for_action_task)
                                .send(Err(status))
                                .await
                                .is_err()
                            {
                                tracing::warn!(
                                    "Failed to send rate limit status to disconnected user {}",
                                    user_uuid
                                );
                                break;
                            }
                            continue;
                        }

                        if let Some(act) = player_action.action {
                            let send_result = process_stream_action()
                                .user_uuid(user_uuid)
                                .action(act)
                                .tx(&user_channel_tx_arc_for_action_task) // Pass a reference to the sender
                                .get_game_tick_uc(get_game_tick_uc.clone())
                                .get_corporation_uc(get_corporation_uc.clone())
                                .list_units_by_corporation_uc(list_units_by_corporation_uc.clone())
                                .spawn_unit_uc(spawn_unit_uc.clone())
                                .acquire_listed_business_uc(acquire_listed_business_uc.clone())
                                .query_business_listings_uc(query_business_listings_uc.clone())
                                .request_uuid(player_action.request_uuid)
                                .call()
                                .await;

                            if send_result.is_err() {
                                tracing::warn!(
                                    "Failed to send action response to disconnected user {}",
                                    user_uuid
                                );
                                break;
                            }
                        } else {
                            tracing::debug!(
                                "Received PlayerAction with empty action field from {}",
                                user_uuid
                            );
                        }
                    }
                    Err(status) => {
                        match status.code() {
                            Code::Unknown => {
                                tracing::debug!(
                                    "Client from user '{}' probably disconnected with status  {}",
                                    user_uuid,
                                    status
                                );
                            }
                            _ => {
                                tracing::error!(
                                    "Error receiving action from user {} with status  {}",
                                    user_uuid,
                                    status
                                );
                            }
                        }

                        let _ = (*user_channel_tx_arc_for_action_task)
                            .send(Err(Status::internal("Stream read error")))
                            .await;

                        break;
                    }
                }
            }
            tracing::info!("Action processing loop finished for user {}", user_uuid);
            // _channel_guard is dropped here
        });

        // Task 2: Listen for and Deliver Results via Pub/Sub
        let valkey_outcome_clone = self.valkey_client.clone();

        tokio::spawn(async move {
            let mut pubsub_conn = match valkey_outcome_clone.get_async_pubsub().await {
                Ok(conn) => conn,
                Err(err) => {
                    tracing::error!(user_id=%user_uuid, error=%err, "Failed to get Valkey PubSub connection");
                    let _ = (*user_channel_tx_arc_for_outcome_task)
                        .send(Err(Status::internal("Outcome listener setup failed")))
                        .await;
                    return;
                }
            };

            let channel_name = create_notification_channel(user_uuid);
            if let Err(err) = pubsub_conn.subscribe(&channel_name).await {
                tracing::error!(user_uuid=%user_uuid, channel=%channel_name, error=%err, "Failed to subscribe to outcome channel");
                let _ = (*user_channel_tx_arc_for_outcome_task)
                    .send(Err(Status::internal("Outcome subscription failed")))
                    .await;
                return;
            }

            tracing::info!(user_uuid=%user_uuid, channel=%channel_name, "Subscribed to outcome notifications");
            let mut message_stream = pubsub_conn.on_message();

            'while_msg: while let Some(msg) = message_stream.next().await {
                let request_uuid_str: String = match msg.get_payload() {
                    Ok(id) => id,
                    Err(err) => {
                        tracing::error!(user_uuid=%user_uuid, error=%err, "Failed to get payload from PubSub message");
                        continue;
                    }
                };

                let request_uuid = match Uuid::parse_str(&request_uuid_str) {
                    Ok(uuid) => uuid,
                    Err(err) => {
                        tracing::error!(user_uuid=%user_uuid, error=%err, raw_uuid=%request_uuid_str, "Failed to parse request uuid from PubSub");
                        continue 'while_msg;
                    }
                };

                tracing::debug!(user_uuid=%user_uuid, request_uuid=%request_uuid, "Received outcome notification");

                match outcome_reader_clone.retrieve_outcome(request_uuid).await {
                    Ok(Some(payload_bytes)) => {
                        let send_res = send_specific_result(
                            &user_channel_tx_arc_for_outcome_task, // Pass reference to sender
                            &payload_bytes,
                            user_uuid,
                        )
                        .await;

                        if send_res.is_err() {
                            tracing::warn!(user_uuid=%user_uuid, request_uuid=%request_uuid, "Failed to send final outcome, client likely disconnected. Terminating outcome listener for this user.");
                            break 'while_msg; // Client disconnected, stop listening for outcomes for this user
                        } else if let Err(err) =
                            outcome_reader_clone.delete_outcome(request_uuid).await
                        {
                            tracing::warn!(user_uuid=%user_uuid, request_uuid=%request_uuid, error=%err, "Failed to delete outcome from store");
                        }
                    }
                    Ok(None) => {
                        tracing::warn!(user_uuid=%user_uuid, request_uuid=%request_uuid, "Outcome payload not found in store");
                    }
                    Err(err) => {
                        tracing::error!(user_uuid=%user_uuid, request_uuid=%request_uuid, error=%err, "Failed to retrieve outcome payload from store");
                        // Optionally send an error to the client if the channel is still open, though this error is server-side.
                        // For now, just log and continue, as the primary issue is data retrieval, not client comms.
                    }
                }
            }
            tracing::info!(user_uuid=%user_uuid, "Outcome listener loop finished.");
        });

        Ok(Response::new(
            Box::pin(response_stream) as Self::PlayStreamStream
        ))
    }
}

#[builder]
async fn process_stream_action<Q, UNT, CRP, GTR, BL>(
    action: Action,
    tx: &UserTx,
    request_uuid: String,
    user_uuid: Uuid,
    get_game_tick_uc: Arc<GetGameTickUseCase<GTR>>,
    get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    list_units_by_corporation_uc: Arc<ListUnitsByCorporationUseCase<UNT>>,
    spawn_unit_uc: Arc<SpawnUnitUseCase<Q, GTR>>,
    acquire_listed_business_uc: Arc<AcquireListedBusinessUseCase<Q, GTR>>,
    query_business_listings_uc: Arc<QueryBusinessListingsUseCase<BL>>,
) -> Result<(), SendError<Result<GameUpdate, Status>>>
where
    Q: ActionQueueable,
    UNT: UnitRepository,
    CRP: CorporationRepository,
    GTR: GameTickRepository,
    BL: BusinessListingRepository,
{
    let Ok(request_uuid) = Uuid::from_str(&request_uuid) else {
        let game_tick = get_game_tick_uc.execute().await.unwrap_or_default();

        let game_update = PresentationError::InvalidArgument("Invalid request UUID".to_string())
            .into_game_update(game_tick, request_uuid.to_string());

        return tx.send(Ok(game_update)).await;
    };

    let result = match action {
        Action::GetCorporation(_) => {
            get_corporation()
                .get_game_tick_uc(get_game_tick_uc)
                .get_corporation_uc(get_corporation_uc)
                .user_uuid(user_uuid)
                .request_uuid(request_uuid)
                .call()
                .await
        }
        Action::SpawnUnit(_) => {
            spawn_unit()
                .get_game_tick_uc(get_game_tick_uc)
                .request_uuid(request_uuid)
                .req_user_uuid(user_uuid)
                .spawn_unit_uc(spawn_unit_uc)
                .call()
                .await
        }
        Action::ListUnit(req) => {
            list_units()
                .get_game_tick_uc(get_game_tick_uc)
                .corporation_uuid(req.corporation_uuid)
                .request_uuid(request_uuid)
                .list_units_by_corporation_uc(list_units_by_corporation_uc)
                .call()
                .await
        }
        Action::AcquireListedBusiness(req) => {
            acquire_listed_business()
                .get_game_tick_uc(get_game_tick_uc)
                .acquire_listed_business_uc(acquire_listed_business_uc)
                .req_user_uuid(user_uuid)
                .request_uuid(request_uuid)
                .business_listing_uuid(req.business_listing_uuid)
                .call()
                .await
        }
        Action::QueryBusinessListings(req) => {
            query_business_listings()
                .get_game_tick_uc(get_game_tick_uc)
                .req(req)
                .request_uuid(request_uuid)
                .query_business_listings_uc(query_business_listings_uc)
                .call()
                .await
        }
    };

    tx.send(result).await
}

async fn send_specific_result(
    tx: &UserTx, // Takes a reference to the mpsc::Sender
    payload_bytes: &[u8],
    user_uuid: Uuid,
) -> Result<(), SendError<Result<GameUpdate, Status>>> {
    match rmp_serde::from_slice::<DomainActionOutcome>(payload_bytes) {
        Ok(outcome) => {
            let game_update = outcome_to_grpc_update(outcome);

            tx.send(Ok(game_update)).await
        }
        Err(err) => {
            tracing::error!(user_uuid=%user_uuid, error=%err, "Failed to deserialize result payload for outcome delivery");
            Ok(())
        }
    }
}

fn outcome_to_grpc_update(outcome: DomainActionOutcome) -> GameUpdate {
    let (update, game_tick) = match outcome {
        DomainActionOutcome::UnitSpawned {
            unit_uuid,
            corporation_uuid,
            tick_effective,
            request_uuid,
            ..
        } => {
            let response = SpawnUnitResponse {
                request_uuid: request_uuid.to_string(),
                unit: Some(Unit {
                    uuid: unit_uuid.to_string(),
                    corporation_uuid: corporation_uuid.to_string(),
                }),
            };
            (Update::SpawnUnit(response), tick_effective)
        }
        DomainActionOutcome::ListedBusinessAcquired {
            request_uuid,
            tick_effective,
            business_uuid,
            market_uuid,
            owning_corporation_uuid,
            name,
            operational_expenses,
            ..
        } => {
            let response = AcquireListedBusinessResponse {
                request_uuid: request_uuid.to_string(),
                business: Some(Business {
                    uuid: business_uuid.to_string(),
                    market_uuid: market_uuid.to_string(),
                    owning_corporation_uuid: owning_corporation_uuid.to_string(),
                    name,
                    operational_expenses,
                }),
            };
            (Update::AcquireListedBusiness(response), tick_effective)
        }
        DomainActionOutcome::CorporationCreated {
            request_uuid,
            tick_effective,
            corporation_uuid,
            user_uuid,
            corporation_name,
            corporation_balance,
            ..
        } => {
            let response = CreateCorporationResponse {
                request_uuid: request_uuid.to_string(),
                corporation: Some(syndicode_proto::syndicode_economy_v1::Corporation {
                    uuid: corporation_uuid.to_string(),
                    user_uuid: user_uuid.to_string(),
                    name: corporation_name,
                    balance: corporation_balance,
                }),
            };

            (Update::CreateCorporation(response), tick_effective)
        }
        DomainActionOutcome::CorporationDeleted {
            tick_effective,
            user_uuid,
            corporation_uuid,
            request_uuid,
            ..
        } => {
            let response = DeleteCorporationResponse {
                request_uuid: request_uuid.to_string(),
                user_uuid: user_uuid.to_string(),
                corporation_uuid: corporation_uuid.to_string(),
            };
            (Update::DeleteCorporation(response), tick_effective)
        }
        DomainActionOutcome::ActionFailed {
            reason,
            tick_processed,
            request_uuid,
            ..
        } => (
            Update::ActionFailedResponse(ActionFailedResponse {
                request_uuid: request_uuid.to_string(),
                reason,
            }),
            tick_processed,
        ),
    };

    GameUpdate {
        game_tick,
        update: Some(update),
    }
}
