use super::{ValkeyStore, ACTION_STREAM_KEY, PAYLOAD_FIELD};
use crate::application::{
    action::QueuedActionPayload,
    ports::queuer::{ActionQueueable, QueueError, QueueResult},
};
use redis::AsyncCommands;

#[tonic::async_trait]
impl ActionQueueable for ValkeyStore {
    /// Enqueues an action payload into a Redis Stream using the XADD command.
    async fn enqueue_action(&self, action: QueuedActionPayload) -> QueueResult<String> {
        let mut conn = self.conn.clone();

        // Use msgpack for potentially better performance/size than JSON
        let action_payload = rmp_serde::to_vec(&action)
            .map_err(|err| QueueError::SerializationError(err.to_string()))?;

        conn.xadd(ACTION_STREAM_KEY, "*", &[(PAYLOAD_FIELD, action_payload)])
            .await
            .map_err(|err| {
                QueueError::EnqueueFailed(format!(
                    "Redis XADD command failed for stream '{ACTION_STREAM_KEY}': {err}"
                ))
            })
    }
}
