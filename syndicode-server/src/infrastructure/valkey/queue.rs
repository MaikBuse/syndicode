use super::ValkeyStore;
use crate::application::{
    action::QueuedAction,
    ports::action_queue::{ActionQueuer, QueueError, QueueResult},
};
use anyhow::Context;
use redis::AsyncCommands;

pub const ACTION_STREAM_KEY: &str = "syndicode:game_actions";
pub const CONSUMER_GROUP_NAME: &str = "leader_processors";
pub const PAYLOAD_FIELD: &str = "payload";

#[tonic::async_trait]
impl ActionQueuer for ValkeyStore {
    /// Enqueues an action payload into a Redis Stream using the XADD command.
    async fn enqueue_action(&self, action_payload: &[u8]) -> QueueResult<String> {
        // Clone the connection handle (cheap). Required because `&mut self` isn't available.
        let mut conn = self.conn.clone();

        // Use XADD to add the entry to the stream.
        // `*` requests Redis to auto-generate the entry ID.
        // We store the payload under a single field named "payload".
        conn.xadd(ACTION_STREAM_KEY, "*", &[(PAYLOAD_FIELD, action_payload)])
            .await
            .map_err(|err| {
                // Map the redis::RedisError to our application-level QueueError
                QueueError::EnqueueFailed(format!(
                    "Redis XADD command failed for stream '{}': {}",
                    ACTION_STREAM_KEY, err
                ))
            })
            .context("Failed to enqueue action to Redis stream")
            .map_err(QueueError::Unexpected)
    }

    /// Pulls actions using XREADGROUP with the ">" ID to get new messages
    /// for the specified consumer.
    async fn pull_actions(&self, count: usize) -> QueueResult<Vec<QueuedAction>> {
        let mut conn = self.conn.clone();

        // Options for XREADGROUP: group name, consumer name, count
        // We don't use BLOCK here, relying on the outer loop's timing.
        // ">" means get entries never delivered to this consumer before.
        let opts = redis::streams::StreamReadOptions::default()
            .group(CONSUMER_GROUP_NAME, self.instance_id)
            .count(count);

        // Execute XREADGROUP
        // The result type can be complex, often involving nested maps or vecs.
        // `redis-rs` typically returns StreamReadReply which contains keys.
        let result: redis::streams::StreamReadReply = conn
            .xread_options(&[ACTION_STREAM_KEY], &[">"], &opts)
            .await
            .map_err(|err| ActionPullError::ConnectionError(format!("XREADGROUP failed: {}", e)))?;

        let mut pulled_actions = Vec::new();

        // Process the reply
        // The reply contains a list of streams (`keys`). We expect only one (stream_key).
        if let Some(stream) = result.keys.into_iter().find(|k| k.key == ACTION_STREAM_KEY) {
            for message in stream.ids {
                let stream_id = message.id;
                // Extract the payload field. Entries are maps (field -> value).
                // Find the field named PAYLOAD_FIELD.
                let payload_value = message.map.get(PAYLOAD_FIELD);

                match payload_value {
                    Some(Value::Data(payload_bytes)) => {
                        // Attempt to deserialize the payload (assuming JSON here)
                        match serde_json::from_slice::<Action>(payload_bytes) {
                            Ok(mut action) => {
                                // Store the stream ID in the action itself for later acknowledgement
                                action.stream_id = stream_id;
                                pulled_actions.push(action);
                            }
                            Err(e) => {
                                // Decide how to handle bad data: log and skip, or fail the batch?
                                // Logging and skipping is often preferred for resilience.
                                tracing::warn!(
                                     stream_id = %stream_id,
                                     error = %e,
                                     "Failed to deserialize action payload, skipping."
                                );
                                // Optionally: Could add logic to move the message to a dead-letter queue
                                // Or immediately ACK it here to remove it if skipping is the policy.
                            }
                        }
                    }
                    Some(_) => {
                        tracing::warn!(stream_id = %stream_id, "Action entry missing or has non-binary data in payload field, skipping.");
                    }
                    None => {
                        tracing::warn!(stream_id = %stream_id, "Action entry missing payload field, skipping.");
                    }
                }
            }
        }
        // If result.keys was empty or didn't contain our stream_key, pulled_actions will be empty.
        Ok(pulled_actions)
    }

    /// Acknowledges processed messages using XACK.
    async fn acknowledge_actions(
        &self,
        ids: &[&str], // Slice of message IDs to acknowledge
    ) -> QueueResult<()> {
        if ids.is_empty() {
            return Ok(()); // Nothing to acknowledge
        }

        let mut conn = self.get_conn().await?;

        // Execute XACK
        let ack_count: i64 = conn
            .xack(stream_key, group_name, ids)
            .await
            .map_err(|e| ActionPullError::ConnectionError(format!("XACK failed: {}", e)))?;

        tracing::debug!(
            acked_count = ack_count,
            expected_count = ids.len(),
            stream = stream_key,
            group = group_name,
            "Acknowledged actions in Redis Stream."
        );

        // XACK returns the number of messages *actually* removed from the PEL.
        // It might be less than ids.len() if some IDs were already acked or invalid,
        // but redis-rs doesn't typically error in that case. A 0 might indicate
        // a problem if you expected to ack messages.
        if ack_count == 0 && !ids.is_empty() {
            tracing::warn!(
                "XACK reported 0 acknowledged messages, expected {}.",
                ids.len()
            );
        } else if ack_count < ids.len() as i64 {
            tracing::warn!("XACK reported {} acknowledged messages, expected {}. Some IDs might have been invalid or already acknowledged.", ack_count, ids.len());
        }

        Ok(())
    }
}
