mod economy;
mod warfare;

use super::common::{ip_address_from_metadata, uuid_from_metadata};
use crate::{
    application::{
        economy::get_corporation::GetCorporationUseCase,
        ports::{
            limiter::{LimiterCategory, RateLimitEnforcer},
            queue::ActionQueuer,
        },
        warfare::{list_units_by_user::ListUnitsByUserUseCase, spawn_unit::SpawnUnitUseCase},
    },
    config::Config,
    domain::{corporation::repository::CorporationRepository, unit::repository::UnitRepository},
    presentation::common::limitation_error_into_status,
};
use dashmap::DashMap;
use economy::get_corporation;
use std::{pin::Pin, sync::Arc};
use syndicode_proto::syndicode_interface_v1::{
    game_service_server::GameService, player_action::Action, GameUpdate, PlayerAction,
};
use tokio::sync::mpsc::{self, error::SendError};
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;
use warfare::{list_units, spawn_unit};

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

pub struct GamePresenter<R, Q, UNT, CRP>
where
    R: RateLimitEnforcer,
    Q: ActionQueuer,
    UNT: UnitRepository,
    CRP: CorporationRepository,
{
    pub config: Arc<Config>,
    pub limit: Arc<R>,
    pub user_channels: UserChannels,
    pub get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    pub list_units_by_user_uc: Arc<ListUnitsByUserUseCase<UNT>>,
    pub spawn_unit_uc: Arc<SpawnUnitUseCase<Q>>,
}

#[tonic::async_trait]
impl<R, Q, UNT, CRP> GameService for GamePresenter<R, Q, UNT, CRP>
where
    R: RateLimitEnforcer + 'static,
    Q: ActionQueuer + 'static,
    UNT: UnitRepository + 'static,
    CRP: CorporationRepository + 'static,
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
        let (tx, rx) = mpsc::channel(16); // Channel for server -> client updates
        let response_stream = ReceiverStream::new(rx);

        // Register User Channel (Potential Race Condition Mitigation)
        // Insert the sender into the map *before* spawning the task.
        // If another part of the system needs to send to this user immediately,
        // the channel might be available slightly sooner.
        if self.user_channels.insert(user_uuid, tx.clone()).is_some() {
            // Optional: Handle case where user was already connected (e.g., kick old session)
            tracing::warn!(
                "User {} connected again, overwriting previous channel.",
                user_uuid
            );
            // Depending on requirements, you might return an error here instead.
            // return Err(Status::already_exists("User already connected"));
        }

        // Clone Arcs needed for the spawned task.
        let get_corporation_uc = Arc::clone(&self.get_corporation_uc);
        let list_units_by_user_uc = Arc::clone(&self.list_units_by_user_uc);
        let spawn_unit_uc = Arc::clone(&self.spawn_unit_uc);

        let limit = Arc::clone(&self.limit);
        let user_channels_clone = Arc::clone(&self.user_channels);

        // Spawn Task to Handle Incoming Client Actions
        tokio::spawn(async move {
            // Create the RAII guard. It owns necessary data for cleanup.
            // Must be created *after* successful insertion into user_channels.
            let _channel_guard = UserChannelGuard {
                user_id: user_uuid,
                channels: user_channels_clone, // Move the cloned Arc here
            };

            let mut stream = request.into_inner(); // The stream of actions from the client

            while let Some(action_result) = stream.next().await {
                match action_result {
                    Ok(action) => {
                        // --- Rate Limiting ---
                        if let Err(err) =
                            limit.check(LimiterCategory::GameStream, &ip_address).await
                        {
                            let status = limitation_error_into_status(err);

                            // Try to send error status. If send fails, client is gone, break loop.
                            if tx.send(Err(status)).await.is_err() {
                                tracing::warn!(
                                    "Failed to send rate limit status to disconnected user {}",
                                    user_uuid
                                );
                                break; // Exit loop if channel is closed
                            }
                            continue; // Skip processing this action
                        }

                        // Action Processing
                        if let Some(request_enum) = action.action {
                            // Use a helper that signals if the channel is closed
                            let send_result = process_action(
                                request_enum,
                                &tx, // Pass borrow for this use case
                                user_uuid,
                                get_corporation_uc.clone(),
                                list_units_by_user_uc.clone(),
                                spawn_unit_uc.clone(),
                            )
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
                        let _ = tx.send(Err(Status::internal("Stream read error"))).await; // Ignore result here
                        break; // Stop processing on stream error
                    }
                }
            }
            // Loop exited (client disconnected, stream error, or explicit break).
            // The _channel_guard's Drop implementation will run here, removing the channel.
            tracing::info!("Action processing loop finished for user {}", user_uuid);
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
async fn process_action<Q, UNT, CRP>(
    action: Action,
    tx: &UserTx, // Borrow the sender channel
    user_uuid: Uuid,
    get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    list_units_by_user_uc: Arc<ListUnitsByUserUseCase<UNT>>,
    spawn_unit_uc: Arc<SpawnUnitUseCase<Q>>,
) -> Result<(), SendError<Result<GameUpdate, Status>>>
where
    Q: ActionQueuer,
    UNT: UnitRepository,
    CRP: CorporationRepository,
{
    let result = match action {
        Action::GetCorporation(req) => get_corporation(req, get_corporation_uc, user_uuid).await,
        Action::SpawnUnit(_) => spawn_unit(spawn_unit_uc, user_uuid).await,
        Action::ListUnit(_) => list_units(list_units_by_user_uc, user_uuid).await,
    };

    // Send the result back to the client
    tx.send(result).await
}
