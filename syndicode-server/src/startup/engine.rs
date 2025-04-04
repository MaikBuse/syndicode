use std::{sync::Arc, time::Duration};
use tokio::{
    sync::Mutex,
    time::{self, Instant},
};

use crate::engine::Engine;

const JOB_INTERVAL: Duration = Duration::from_secs(3);

pub(super) async fn engine_loop(engine: Arc<Mutex<Engine>>) {
    let mut interval = time::interval(JOB_INTERVAL);
    loop {
        let next_tick = Instant::now() + JOB_INTERVAL;

        let engine_clone = Arc::clone(&engine);

        // spawn job without waiting
        tokio::spawn(async move {
            {
                let mut engine = engine_clone.lock().await;
                if let Err(err) = engine.advance_epoch().await {
                    tracing::error!("{}", err);
                }
            }
        });

        // wait until exactly one second from the last tick
        interval.tick().await;
        tokio::time::sleep_until(next_tick).await;
    }
}
