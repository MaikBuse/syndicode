use crate::domain::response::{DomainResponse, ResponseType};
use std::sync::Arc;
use syndicode_proto::syndicode_interface_v1::{game_update::Update, GameUpdate};
use time::OffsetDateTime;
use tokio::{
    select,
    sync::{mpsc, Mutex, Notify},
};
use tokio_stream::StreamExt;
use tonic::Streaming;

use super::app::AppEvent;

#[derive(Debug)]
pub struct StreamHandler {
    start_processing_signal: Arc<Notify>,
    shutdown_signal: Arc<Notify>,
    is_processing: Arc<Mutex<bool>>,
    maybe_server_updates_stream: Arc<Mutex<Option<Streaming<GameUpdate>>>>,
}

impl StreamHandler {
    pub fn new(shutdown_signal: Arc<Notify>) -> Self {
        Self {
            start_processing_signal: Arc::new(Notify::new()),
            maybe_server_updates_stream: Arc::new(Mutex::new(None)),
            is_processing: Arc::new(Mutex::new(false)),
            shutdown_signal,
        }
    }

    // Method to set the stream after construction
    // Takes &self, is async, modifies content of Mutex
    pub async fn set_server_updates_stream(&self, stream: Streaming<GameUpdate>) {
        let mut guard = self.maybe_server_updates_stream.lock().await;
        *guard = Some(stream);
        tracing::debug!("Server updates stream has been set.");
    }

    // Method to signal the spawned task(s) to start processing
    pub fn signal_start_processing(&self) {
        tracing::info!("Signaling tasks to start processing.");
        self.start_processing_signal.notify_one(); // Wakes one waiting task
    }

    // Method that spawns the Tokio task to listen for server updates.
    pub fn spawn_server_updates_listener(
        &self,
        app_event_tx: mpsc::Sender<AppEvent>, // Channel to send responses back
    ) -> tokio::task::JoinHandle<()> {
        let start_signal_clone = Arc::clone(&self.start_processing_signal);
        let shutdown_signal_clone = Arc::clone(&self.shutdown_signal);

        let is_processing_clone = Arc::clone(&self.is_processing);

        // Clone the Arc for the stream. The task will own this clone.
        let server_stream_arc_opt = self.maybe_server_updates_stream.clone();

        tokio::spawn(async move {
            tracing::debug!("Update listener spawned. Waiting for start signal...");

            select! {
                _ = shutdown_signal_clone.notified() => {
                    // Received shutdown signal. Return from the function
                    tracing::debug!("Shutdown signal received. Stopping waiting on start signal.");
                    return;
                }
                _ = start_signal_clone.notified() => {}
            }

            *is_processing_clone.lock().await = true;
            tracing::debug!("Start signal received. Beginning processing of server updates.");

            let mut stream_arc = match server_stream_arc_opt.lock().await.take() {
                Some(arc) => arc,
                None => {
                    let error_response = DomainResponse::builder()
                        .response_type(ResponseType::Error) // Use an appropriate error type
                        .code("STREAM_NOT_CONFIGURED".to_string())
                        .message(
                            "Server updates stream was not available for processing.".to_string(),
                        )
                        .timestamp(OffsetDateTime::now_utc())
                        .build();

                    // Try to send the error back; if it fails, the receiver is gone anyway.
                    app_event_tx
                        .send(AppEvent::StreamUpdate(error_response))
                        .await
                        .unwrap();
                    return;
                }
            };

            // Main loop for processing stream updates
            loop {
                tokio::select! {
                    _ = shutdown_signal_clone.notified() => {
                        tracing::debug!("Shutdown signal received. Stopping processing of server updates.");
                        break;
                    }
                    maybe_stream_result = stream_arc.next() => {
                        if let Some(stream_result) = maybe_stream_result {
                            match stream_result {
                                Ok(game_update) => {
                                    if let Err(err) = handle_game_update(game_update, app_event_tx.clone()).await {
                                        tracing::error!("{}", err);
                                        // Don't break here - just log and continue
                                        continue;
                                    }
                                }
                                Err(status) => {
                                    let error = format!(
                                        "Error receiving game update from stream: {status:?}"
                                    );
                                    tracing::warn!(error);
                                    let response = DomainResponse::builder()
                                        .response_type(ResponseType::Error)
                                        .code(status.code().to_string())
                                        .message(format!("{:#?}", status.message()))
                                        .timestamp(OffsetDateTime::now_utc())
                                        .build();

                                    if app_event_tx
                                        .send(AppEvent::StreamUpdate(response))
                                        .await
                                        .is_err()
                                    {
                                        tracing::error!("Receiver for responses has been dropped (after stream error). Task stopping.");
                                        break;
                                    }
                                    // Don't break here either - continue processing
                                    continue;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                }
            }

            *is_processing_clone.lock().await = false;

            tracing::debug!("Finished processing server updates.");
        })
    }
}

async fn handle_game_update(
    game_update: GameUpdate,
    app_event_tx: mpsc::Sender<AppEvent>,
) -> anyhow::Result<()> {
    let Some(update) = game_update.update.as_ref() else {
        return Err(anyhow::anyhow!("Failed to retrieve Update from GameUpdate"));
    };

    let response = match update {
        Update::TickNotification(_) => DomainResponse::builder()
            .response_type(ResponseType::GameTickeNotification)
            .code("OK".to_string())
            .message(format!("{game_update:#?}"))
            .timestamp(OffsetDateTime::now_utc())
            .build(),
        Update::ActionFailedResponse(_) => DomainResponse::builder()
            .response_type(ResponseType::Error)
            .code("ERR".to_string())
            .message(format!("{game_update:#?}"))
            .timestamp(OffsetDateTime::now_utc())
            .build(),
        _ => DomainResponse::builder()
            .response_type(ResponseType::Info)
            .code("OK".to_string())
            .message(format!("{game_update:#?}"))
            .timestamp(OffsetDateTime::now_utc())
            .build(),
    };

    if app_event_tx
        .send(AppEvent::StreamUpdate(response))
        .await
        .is_err()
    {
        return Err(anyhow::anyhow!(
            "Receiver for responses has been dropped. Task stopping."
        ));
    }

    Ok(())
}
