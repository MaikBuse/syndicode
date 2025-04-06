use super::ValkeyStore;
use crate::application::ports::action_queue::{ActionQueuer, QueueError, QueueResult};
use anyhow::Context;
use redis::AsyncCommands;

const ACTION_KEY: &str = "syndicode:game_actions";

#[tonic::async_trait]
impl ActionQueuer for ValkeyStore {
    /// Enqueues an action payload into a Redis Stream using the XADD command.
    async fn enqueue_action(&self, action_payload: &[u8]) -> QueueResult<String> {
        // Clone the connection handle (cheap). Required because `&mut self` isn't available.
        let mut conn = self.conn.clone();

        // Use XADD to add the entry to the stream.
        // `*` requests Redis to auto-generate the entry ID.
        // We store the payload under a single field named "payload".
        conn.xadd(ACTION_KEY, "*", &[("payload", action_payload)])
            .await
            .map_err(|err| {
                // Map the redis::RedisError to our application-level QueueError
                QueueError::EnqueueFailed(format!(
                    "Redis XADD command failed for stream '{}': {}",
                    ACTION_KEY, err
                ))
            })
            .context("Failed to enqueue action to Redis stream")
            .map_err(QueueError::Unexpected)
    }
}
