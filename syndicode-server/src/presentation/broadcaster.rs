use super::game::UserChannels;
use crate::{
    infrastructure::valkey::outcome::GAME_TICK_NOTIFICATION_CHANNEL, utils::timestamp_now,
};
use bon::Builder;
use futures::stream::{iter, StreamExt};
use std::sync::Arc;
use syndicode_proto::syndicode_interface_v1::{game_update::Update, GameUpdate, TickNotification};

// Configuration for concurrency
const MAX_CONCURRENT_TICK_BROADCASTS: usize = 50;

#[derive(Builder)]
pub struct GameTickBroadcaster {
    pub valkey_client: redis::Client,
    pub user_channels: UserChannels,
}

impl GameTickBroadcaster {
    /// Spawns a background task to listen for tick notifications and broadcast them concurrently.
    /// Returns immediately after spawning. Errors during setup or runtime are logged internally.
    pub fn spawn_listen_and_broadcast_task(&self) {
        // Clone resources needed for the background task
        let valkey_game_tick_clone = self.valkey_client.clone();
        let user_channels_clone = Arc::clone(&self.user_channels);

        tokio::spawn(async move {
            // --- Pub/Sub Setup ---
            let mut pubsub_conn = match valkey_game_tick_clone.get_async_pubsub().await {
                Ok(conn) => conn,
                Err(err) => {
                    // Log error and exit the task if connection fails.
                    tracing::error!(error=%err, "Broadcaster: Failed to get Redis PubSub connection. Task exiting.");
                    return;
                }
            };

            let channel_name = GAME_TICK_NOTIFICATION_CHANNEL;
            if let Err(err) = pubsub_conn.subscribe(&channel_name).await {
                // Log error and exit the task if subscription fails.
                tracing::error!(error=%err, "Broadcaster: Failed to subscribe to game tick notification channel '{}'. Task exiting.", channel_name);
                return;
            }

            tracing::info!(
                "Broadcaster: Subscribed to game tick notifications on channel '{}'",
                channel_name
            );

            let mut message_stream = pubsub_conn.on_message();

            // --- Main Listener Loop ---
            while let Some(msg) = message_stream.next().await {
                let game_tick: i64 = match msg.get_payload() {
                    Ok(gt) => gt,
                    Err(err) => {
                        tracing::error!(error = %err, "Broadcaster: Failed to get payload from tick PubSub message. Skipping.");
                        continue;
                    }
                };

                // Get timestamp *once* per received tick event
                let Ok(effective_at_ts) = timestamp_now() else {
                    tracing::error!(tick=%game_tick, "Broadcaster: Failed to generate timestamp. Skipping broadcast.");
                    continue;
                };

                tracing::debug!(tick = %game_tick, num_local_clients = user_channels_clone.len(), "Broadcaster: Received tick notification. Broadcasting...");

                // --- Construct the simple update message ONCE ---
                // (Since we are *not* fetching player-specific state yet, the message is the same for everyone)
                let update_payload = GameUpdate {
                    game_tick,
                    update: Some(Update::TickNotification(TickNotification {
                        effective_at: Some(effective_at_ts),
                    })),
                };

                // --- Concurrent Broadcasting ---
                let channels_iter = user_channels_clone.iter().map(|entry| {
                    // Only need to clone the Sender for the async block now
                    entry.value().clone()
                });

                iter(channels_iter)
                    .for_each_concurrent(MAX_CONCURRENT_TICK_BROADCASTS, |user_tx| {
                        // Clone the update payload for each concurrent task
                        let update_payload_clone = update_payload.clone();

                        async move {
                            // --- Send update ---
                            if let Err(err) = user_tx.send(Ok(update_payload_clone)).await {
                                // Log error if sending failed (client likely disconnected/channel full)
                                // Note: We don't have the user_uuid easily here without cloning it too.
                                tracing::warn!(error=%err, tick=%game_tick, "Broadcaster: Failed to send TickNotification update to a client channel.");
                            }
                        } // End async move block
                    }) // End for_each_concurrent
                    .await; // Wait for this tick's broadcast batch to finish

                tracing::debug!(tick=%game_tick, "Broadcaster: Finished broadcasting tick.");
            } // End while let Some(msg)

            tracing::info!(
                "Broadcaster: Game tick notification listener loop finished unexpectedly."
            );
            // Consider what happens if the loop exits. Should the application shut down?
            // Does it indicate an unrecoverable error with Redis?
        }); // End tokio::spawn
    }
}
