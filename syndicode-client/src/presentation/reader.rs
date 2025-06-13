use std::{sync::Arc, time::Duration};

use super::app::AppEvent;
use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, Notify};

#[derive(Debug)]
pub struct Reader {
    shutdown_signal: Arc<Notify>,
    app_event_tx: mpsc::Sender<AppEvent>,
}

impl Reader {
    pub fn new(shutdown_signal: Arc<Notify>, app_event_tx: mpsc::Sender<AppEvent>) -> Self {
        Reader {
            shutdown_signal,
            app_event_tx,
        }
    }

    pub async fn poll_events(self, tx: mpsc::Sender<AppEvent>, shutdown_signal: Arc<Notify>) {
        let mut reader = crossterm::event::EventStream::new();
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        loop {
            let delay = interval.tick().fuse();
            let crossterm_event = reader.next().fuse();
            tokio::select! {
                _ = shutdown_signal.notified() => {
                    tracing::debug!("Shutdown signal received. Stopping reading input events.");
                    break;
                }
                maybe_event = crossterm_event => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            if tx.send(AppEvent::Crossterm(event)).await.is_err() {
                                break;
                            }
                        }
                        Some(Err(_)) | None => {
                            break;
                        }
                    }
                }
                _ = delay => {}
            }
        }
    }

    pub fn spawn_read_input_events(self) -> tokio::task::JoinHandle<()> {
        let shutdown_signal = self.shutdown_signal.clone();
        let app_event_tx = self.app_event_tx.clone();
        tokio::spawn(async move {
            self.poll_events(app_event_tx, shutdown_signal).await;
        })
    }
}
