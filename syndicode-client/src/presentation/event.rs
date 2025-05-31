use std::sync::Arc;

use super::app::AppEvent;
use tokio::sync::{mpsc, Notify};

#[derive(Debug)]
pub struct InputReader {
    shutdown_signal: Arc<Notify>,
}

impl InputReader {
    pub fn new(shutdown_signal: Arc<Notify>) -> Self {
        InputReader { shutdown_signal }
    }

    pub async fn read_input_events(self, tx: mpsc::Sender<AppEvent>) {
        'read_input_loop: loop {
            let event_fut = tokio::task::spawn_blocking(ratatui::crossterm::event::read);
            tokio::select! {
                _ = self.shutdown_signal.notified() => {
                    // Received shutdown signal, break the loop
                    break 'read_input_loop;
                }
                event_result = event_fut => {
                    match event_result {
                        Ok(Ok(event)) => {
                            if tx.send(AppEvent::Crossterm(event)).await.is_err() {
                                break;
                            }
                        }
                        Ok(Err(_io_err)) => {
                            break;
                        }
                        Err(_join_err) => {
                            break;
                        }
                    }
                }
            }
        }
    }
}
