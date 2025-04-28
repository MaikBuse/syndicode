mod economy;
mod warfare;

use super::common::{ip_address_from_metadata, uuid_from_metadata};
use crate::{
    application::{
        economy::{
            acquire_listed_business::AcquireListedBusinessUseCase,
            get_corporation::GetCorporationUseCase,
            query_business_listings::QueryBusinessListingsUseCase,
        },
        ports::{
            game_tick::GameTickRepository,
            limiter::{LimiterCategory, RateLimitEnforcer},
            outcome::OutcomeStoreReader,
            queuer::ActionQueueable,
        },
        warfare::{list_units_by_user::ListUnitsByUserUseCase, spawn_unit::SpawnUnitUseCase},
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
use dashmap::DashMap;
use economy::{acquire_listed_business, get_corporation, query_business_listings};
use std::{pin::Pin, sync::Arc};
use syndicode_proto::{
    syndicode_economy_v1::{AcquireListedBusinessResponse, Business},
    syndicode_interface_v1::{
        game_service_server::GameService, game_update::Update, player_action::Action,
        ActionFailedResponse, GameUpdate, PlayerAction,
    },
    syndicode_warfare_v1::{SpawnUnitResponse, Unit},
};
use tokio::sync::mpsc::{self, error::SendError};
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;
use warfare::{list_units, spawn_unit};

const MPSC_CHANNEL_BUFFER: usize = 16;

// Type alias for the sender part of the channel for game updates to a specific user.
type UserTx = mpsc::Sender<Result<GameUpdate, Status>>;
// Type alias for the map storing user channels, keyed by user UUID.
type UserChannels = Arc<DashMap<Uuid, UserTx>>;

/// RAII Guard for Channel Cleanup
struct UserChannelGuard {
    user_id: Uuid,
    channels: UserChannels,
}

impl Drop for UserChannelGuard {
    fn drop(&mut self) {
        tracing::debug!(
            "Client disconnected or stream ended for user {}. Removing channel.",
            self.user_id
        );
        // Remove the user's channel sender when the guard is dropped.
        // This happens when the associated task finishes (e.g., client disconnects).
        self.channels.remove(&self.user_id);
    }
}

#[derive(Builder)]
pub struct GamePresenter<R, Q, UNT, CRP, RSR, GTR, BL>
where
    R: RateLimitEnforcer,
    Q: ActionQueueable,
    UNT: UnitRepository,
    CRP: CorporationRepository,
    RSR: OutcomeStoreReader,
    GTR: GameTickRepository,
    BL: BusinessListingRepository,
{
    pub config: Arc<Config>,
    pub valkey_client: redis::Client,
    pub limit: Arc<R>,
    pub result_store_reader: Arc<RSR>,
    pub user_channels: UserChannels,
    pub get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    pub list_units_by_user_uc: Arc<ListUnitsByUserUseCase<UNT>>,
    pub spawn_unit_uc: Arc<SpawnUnitUseCase<Q, GTR>>,
    pub acquire_listed_business_uc: Arc<AcquireListedBusinessUseCase<Q, GTR>>,
    pub query_business_listings_uc: Arc<QueryBusinessListingsUseCase<BL>>,
}

#[tonic::async_trait]
impl<R, Q, UNT, CRP, RSR, GTR, BL> GameService for GamePresenter<R, Q, UNT, CRP, RSR, GTR, BL>
where
    R: RateLimitEnforcer + 'static,
    Q: ActionQueueable + 'static,
    UNT: UnitRepository + 'static,
    CRP: CorporationRepository + 'static,
    RSR: OutcomeStoreReader + 'static,
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

        let user_uuid = uuid_from_metadata(request.metadata())?; // Propagates error status

        // Setup Communication Channel
        let (tx, rx) = mpsc::channel(MPSC_CHANNEL_BUFFER); // Channel for server -> client updates
        let response_stream = ReceiverStream::new(rx);

        // Register User Channel (Potential Race Condition Mitigation)
        // Insert the sender into the map *before* spawning the task.
        // If another part of the system needs to send to this user immediately,
        // the channel might be available slightly sooner.
        if self.user_channels.insert(user_uuid, tx.clone()).is_some() {
            tracing::warn!(
                "User {} connected again, overwriting previous channel.",
                user_uuid
            );
        }

        // Clone Arcs needed for the spawned task.
        let get_corporation_uc = Arc::clone(&self.get_corporation_uc);
        let list_units_by_user_uc = Arc::clone(&self.list_units_by_user_uc);
        let acquire_listed_business_uc = Arc::clone(&self.acquire_listed_business_uc);
        let query_business_listings_uc = Arc::clone(&self.query_business_listings_uc);
        let spawn_unit_uc = Arc::clone(&self.spawn_unit_uc);

        let limit = Arc::clone(&self.limit);
        let user_channels_clone = Arc::clone(&self.user_channels);
        let result_reader_clone = Arc::clone(&self.result_store_reader);

        let tx_clone_for_action_task = tx.clone(); // Clone Tx for action processing task
        let tx_clone_for_result_task = tx.clone(); // Clone Tx for result listener task

        // Task 1: Handle Incoming Client Actions
        tokio::spawn(async move {
            // Create the RAII guard. It owns necessary data for cleanup.
            // Must be created *after* successful insertion into user_channels.
            let _channel_guard = UserChannelGuard {
                user_id: user_uuid,
                channels: user_channels_clone, // Move the cloned Arc here
            };

            let mut stream = request.into_inner(); // The stream of actions from the client

            while let Some(stream_result) = stream.next().await {
                match stream_result {
                    Ok(player_action) => {
                        // --- Rate Limiting ---
                        if let Err(err) =
                            limit.check(LimiterCategory::GameStream, &ip_address).await
                        {
                            let status = limitation_error_into_status(err);

                            // Try to send error status. If send fails, client is gone, break loop.
                            if tx_clone_for_action_task.send(Err(status)).await.is_err() {
                                tracing::warn!(
                                    "Failed to send rate limit status to disconnected user {}",
                                    user_uuid
                                );
                                break; // Exit loop if channel is closed
                            }
                            continue; // Skip processing this action
                        }

                        // Action Processing
                        let Ok(request_uuid) = Uuid::parse_str(&player_action.request_uuid) else {
                            let _ = tx_clone_for_action_task
                                .send(Err(Status::internal(
                                    "Failed to parse the provided request uuid",
                                )))
                                .await;

                            continue;
                        };

                        if let Some(act) = player_action.action {
                            // Use a helper that signals if the channel is closed
                            let send_result = process_action()
                                .user_uuid(user_uuid)
                                .action(act)
                                .tx(&tx_clone_for_action_task)
                                .get_corporation_uc(get_corporation_uc.clone())
                                .list_units_by_user_uc(list_units_by_user_uc.clone())
                                .spawn_unit_uc(spawn_unit_uc.clone())
                                .acquire_listed_business_uc(acquire_listed_business_uc.clone())
                                .query_business_listings_uc(query_business_listings_uc.clone())
                                .request_uuid(request_uuid)
                                .call()
                                .await;

                            // If sending failed (channel closed), break the loop.
                            if send_result.is_err() {
                                tracing::warn!(
                                    "Failed to send response to disconnected user {}",
                                    user_uuid
                                );
                                break;
                            }
                        } else {
                            // Optional: Handle empty action payload if necessary
                            tracing::debug!(
                                "Received PlayerAction with empty action field from {}",
                                user_uuid
                            );
                        }
                    }
                    Err(status) => {
                        // Error reading from the client stream
                        tracing::error!(
                            "Error receiving action from user {}: {}",
                            user_uuid,
                            status
                        );
                        // Attempt to notify the client if possible, though the stream is likely broken.
                        let _ = tx_clone_for_action_task
                            .send(Err(Status::internal("Stream read error")))
                            .await; // Ignore result here
                        break; // Stop processing on stream error
                    }
                }
            }

            // Loop exited (client disconnected, stream error, or explicit break).
            // The _channel_guard's Drop implementation will run here, removing the channel.
            tracing::info!("Action processing loop finished for user {}", user_uuid);
        });

        // Task 2: Listen for and Deliver Results via Pub/Sub
        let valkey_client_clone = self.valkey_client.clone();

        tokio::spawn(async move {
            // Subscribe to the client's result channel
            let mut pubsub_conn = match valkey_client_clone.get_async_pubsub().await {
                Ok(conn) => conn,
                Err(err) => {
                    tracing::error!(user_id=%user_uuid, error=%err, "Failed to get Redis PubSub connection");
                    // Attempt to send error to client if channel still open
                    let _ = tx_clone_for_result_task
                        .send(Err(Status::internal("Result listener setup failed")))
                        .await;
                    return; // Exit task
                }
            };

            let channel_name = create_notification_channel(user_uuid);
            if let Err(err) = pubsub_conn.subscribe(&channel_name).await {
                tracing::error!(user_uuid=%user_uuid, channel=%channel_name, error=%err, "Failed to subscribe to result channel");
                let _ = tx_clone_for_result_task
                    .send(Err(Status::internal("Result subscription failed")))
                    .await;
                return; // Exit task
            }

            tracing::info!(user_uuid=%user_uuid, channel=%channel_name, "Subscribed to result notifications");

            // Use a Stream adapter for the pubsub messages
            let mut message_stream = pubsub_conn.on_message();

            'while_msg: while let Some(msg) = message_stream.next().await {
                let request_uuid: String = match msg.get_payload() {
                    Ok(id) => id,
                    Err(err) => {
                        tracing::error!(user_uuid=%user_uuid, error=%err, "Failed to get payload from PubSub message");
                        continue; // Skip malformed message
                    }
                };

                let request_uuid = match Uuid::parse_str(&request_uuid) {
                    Ok(request_uuid) => request_uuid,
                    Err(err) => {
                        tracing::error!(user_uuid=%user_uuid, error=%err, "Failed to parse request uuid");

                        continue 'while_msg;
                    }
                };

                tracing::debug!(user_uuid=%user_uuid, request_uuid=%request_uuid, "Received result notification");

                // Fetch the full result from the store
                match result_reader_clone.retrieve_outcome(request_uuid).await {
                    Ok(Some(payload_bytes)) => {
                        // Attempt to deserialize and construct the GameUpdate
                        // This helper needs access to the Tx channel
                        let send_res = send_specific_result(
                            &tx_clone_for_result_task,
                            &payload_bytes,
                            user_uuid,
                        )
                        .await;

                        if send_res.is_err() {
                            tracing::warn!(user_uuid=%user_uuid, request_uuid=%request_uuid, "Failed to send final result, client likely disconnected.");
                            // No need to break here, subscription might still be valid, but the specific send failed.
                            // Consider breaking if the send error indicates a permanently closed channel.
                        } else {
                            // Delete the result from store after successful send attempt
                            if let Err(err) = result_reader_clone.delete_outcome(request_uuid).await
                            {
                                tracing::warn!(user_uuid=%user_uuid, request_uuid=%request_uuid, error=%err, "Failed to delete result from store");
                            }
                        }
                    }
                    Ok(None) => {
                        tracing::warn!(user_uuid=%user_uuid, request_uuid=%request_uuid, "Result payload not found in store (TTL expired or deleted?)");
                    }
                    Err(err) => {
                        tracing::error!(user_uuid=%user_uuid, request_uuid=%request_uuid, error=%err, "Failed to retrieve result payload from store");
                    }
                }
            } // End while let Some(msg)

            tracing::info!(user_uuid=%user_uuid, "Result listener loop finished.");
            // Unsubscribe happens automatically when pubsub_conn is dropped
        });

        // - Return the Response Stream
        // Box::pin is necessary to create the trait object Stream.
        Ok(Response::new(
            Box::pin(response_stream) as Self::PlayStreamStream
        ))
    }
}

/// Processes a single action and sends the result/error back through the channel.
/// Returns Ok(()) if sending succeeded, Err(()) if the channel was closed.
#[builder]
async fn process_action<Q, UNT, CRP, GTR, BL>(
    action: Action,
    tx: &UserTx, // Borrow the sender channel
    request_uuid: Uuid,
    user_uuid: Uuid,
    get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    list_units_by_user_uc: Arc<ListUnitsByUserUseCase<UNT>>,
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
    let result = match action {
        Action::GetCorporation(_) => {
            get_corporation()
                .get_corporation_uc(get_corporation_uc)
                .user_uuid(user_uuid)
                .request_uuid(request_uuid)
                .call()
                .await
        }
        Action::SpawnUnit(_) => {
            spawn_unit()
                .request_uuid(request_uuid)
                .req_user_uuid(user_uuid)
                .spawn_unit_uc(spawn_unit_uc)
                .call()
                .await
        }
        Action::ListUnit(_) => {
            list_units()
                .req_user_uuid(user_uuid)
                .request_uuid(request_uuid)
                .list_units_by_user_uc(list_units_by_user_uc)
                .call()
                .await
        }
        Action::AcquireListedBusiness(req) => {
            async {
                let business_uuid = Uuid::parse_str(&req.business_uuid)
                    .map_err(|_| Status::invalid_argument("Failed to parse business_uuid"))?;

                acquire_listed_business()
                    .acquire_listed_business_uc(acquire_listed_business_uc)
                    .req_user_uuid(user_uuid)
                    .request_uuid(request_uuid)
                    .business_uuid(business_uuid)
                    .call()
                    .await
            }
            .await
        }
        Action::QueryBusinessListings(req) => {
            query_business_listings()
                .req(req)
                .request_uuid(request_uuid)
                .query_business_listings_uc(query_business_listings_uc)
                .call()
                .await
        }
    };

    // Send the result back to the client
    tx.send(result).await
}

// Helper function to deserialize and send the specific result
async fn send_specific_result(
    tx: &UserTx,
    payload_bytes: &[u8],
    user_uuid: Uuid, // For logging
) -> Result<(), SendError<Result<GameUpdate, Status>>> {
    // Deserialize based on how it was stored (e.g., DomainActionOutcome)

    match rmp_serde::from_slice::<DomainActionOutcome>(payload_bytes) {
        Ok(outcome) => {
            // Convert Domain Outcome to gRPC GameUpdate
            let game_update = outcome_to_grpc_update(outcome);
            // Assume this helper exists
            tx.send(Ok(game_update)).await
        }
        Err(err) => {
            tracing::error!(user_uuid=%user_uuid, error=%err, "Failed to deserialize result payload");
            // Don't send error back here, as the client didn't explicitly ask for this update
            // Just log the failure.
            Ok(()) // Don't propagate deserialization error as a channel send error
        }
    }
}

// Helper to convert Domain outcome -> gRPC Update
fn outcome_to_grpc_update(outcome: DomainActionOutcome) -> GameUpdate {
    let (update, game_tick, request_uuid) = match outcome {
        // Assume outcome includes tick if needed
        DomainActionOutcome::UnitSpawned {
            user_uuid,
            unit_uuid,
            tick_effective,
            request_uuid,
            ..
        } => {
            let response = SpawnUnitResponse {
                unit: Some(Unit {
                    uuid: unit_uuid.to_string(),
                    user_uuid: user_uuid.to_string(),
                }),
            };
            (Update::SpawnUnit(response), tick_effective, request_uuid)
        }
        DomainActionOutcome::ListedBusinessAcquired {
            request_uuid,
            tick_effective,
            user_uuid: _,
            business_uuid,
            market_uuid,
            owning_corporation_uuid,
            name,
            operational_expenses,
        } => {
            let response = AcquireListedBusinessResponse {
                business: Some(Business {
                    uuid: business_uuid.to_string(),
                    market_uuid: market_uuid.to_string(),
                    owning_corporation_uuid: owning_corporation_uuid.to_string(),
                    name,
                    operational_expenses,
                }),
            };

            (
                Update::AcquireListedBusiness(response),
                tick_effective,
                request_uuid,
            )
        }
        DomainActionOutcome::ActionFailed {
            reason,
            tick_processed,
            request_uuid,
            ..
        } => (
            Update::ActionFailedResponse(ActionFailedResponse { reason }),
            tick_processed,
            request_uuid,
        ),
    };

    GameUpdate {
        request_uuid: request_uuid.to_string(),
        game_tick,
        update: Some(update),
    }
}
