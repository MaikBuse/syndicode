use super::app::AppEvent;
use ratatui::crossterm::event::{Event, KeyCode};
use tokio::sync::mpsc;

pub struct InputReader {
    just_pressed_esc: bool,
    should_exit: bool,
}

impl InputReader {
    pub fn new() -> Self {
        InputReader {
            just_pressed_esc: false,
            should_exit: false,
        }
    }

    pub async fn read_input_events(mut self, tx: mpsc::Sender<AppEvent>) {
        'read_input_loop: loop {
            // Spawn the blocking read operation on a dedicated thread
            // so it doesn't block the async runtime.
            let event_result = tokio::task::spawn_blocking(ratatui::crossterm::event::read).await;

            match event_result {
                Ok(Ok(event)) => {
                    if let Event::Key(key_event) = event {
                        if self.just_pressed_esc && key_event.code == KeyCode::Char('y') {
                            self.should_exit = true;
                        }

                        self.just_pressed_esc = key_event.code == KeyCode::Esc;
                    }

                    // spawn_blocking finished successfully, and read() was Ok
                    if tx.send(AppEvent::Crossterm(event)).await.is_err() {
                        // Receiver (event_rx in main) has been dropped,
                        // indicating the main app loop has exited. So, we should also exit.
                        break;
                    }

                    if self.should_exit {
                        break 'read_input_loop;
                    }
                }
                Ok(Err(_io_err)) => {
                    // crossterm::event::read() returned an IO error.
                    break; // Exit loop on IO error
                }
                Err(_join_err) => {
                    // The spawn_blocking task panicked or was cancelled (e.g. by runtime shutdown).
                    break; // Exit loop if the blocking task fails
                }
            }
        }
    }
}
