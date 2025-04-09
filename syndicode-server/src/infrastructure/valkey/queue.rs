use super::ValkeyStore;
use crate::application::{
    action::QueuedAction,
    ports::queue::{ActionQueuer, QueueError, QueueResult},
};
use redis::{streams::StreamReadReply, AsyncCommands, Value};

pub const ACTION_STREAM_KEY: &str = "syndicode:game_actions";
pub const ACTION_CONSUMER_GROUP: &str = "leader_processors";
pub const PAYLOAD_FIELD: &str = "payload";
const BATCH_PULL_SIZE: usize = 100; // How many messages to request per internal batch

// Private helper function to avoid code duplication
impl ValkeyStore {
    /// Fetches a batch of actions using XREADGROUP. Internal helper.
    /// Returns message IDs along with actions.
    async fn pull_actions_batch(&self, count: usize) -> QueueResult<Vec<(String, QueuedAction)>> {
        let mut conn = self.conn.clone();

        let opts = redis::streams::StreamReadOptions::default()
            .group(ACTION_CONSUMER_GROUP, self.instance_id.clone())
            .count(count);

        let result: StreamReadReply = conn
            .xread_options(&[ACTION_STREAM_KEY], &[">"], &opts)
            .await
            .map_err(|err| QueueError::ConnectionError(format!("XREADGROUP failed: {}", err)))?;

        let mut pulled_actions = Vec::new();

        if let Some(stream) = result.keys.into_iter().find(|k| k.key == ACTION_STREAM_KEY) {
            if stream.ids.is_empty() {
                // No messages in this batch for this stream
                return Ok(pulled_actions);
            }

            for message in stream.ids {
                let stream_id = message.id; // Keep the ID
                let payload_value = message.map.get(PAYLOAD_FIELD);

                match payload_value {
                    Some(Value::BulkString(payload_bytes)) => {
                        match rmp_serde::from_slice::<QueuedAction>(payload_bytes) {
                            Ok(action) => {
                                // Store both ID and deserialized action
                                pulled_actions.push((stream_id.clone(), action));
                            }
                            Err(err) => {
                                tracing::warn!(
                                     stream_id = %stream_id,
                                     error = %err,
                                     "Failed to deserialize action payload, skipping message."
                                );
                                // Optional: Ack bad message immediately? Move to DLQ?
                            }
                        }
                    }
                    Some(_) => {
                        tracing::warn!(stream_id = %stream_id, "Action entry has non-binary data in payload field, skipping message.");
                    }
                    None => {
                        tracing::warn!(stream_id = %stream_id, "Action entry missing payload field, skipping message.");
                    }
                }
            }
        }
        Ok(pulled_actions)
    }
}

#[tonic::async_trait]
impl ActionQueuer for ValkeyStore {
    /// Enqueues an action payload into a Redis Stream using the XADD command.
    async fn enqueue_action(&self, action: QueuedAction) -> QueueResult<String> {
        let mut conn = self.conn.clone();

        // Use msgpack for potentially better performance/size than JSON
        let action_payload = rmp_serde::to_vec(&action)
            .map_err(|err| QueueError::SerializationError(err.to_string()))?;
        // Removed anyhow::format_err! usage for direct QueueError variant

        conn.xadd(ACTION_STREAM_KEY, "*", &[(PAYLOAD_FIELD, action_payload)])
            .await
            .map_err(|err| {
                QueueError::EnqueueFailed(format!(
                    "Redis XADD command failed for stream '{}': {}",
                    ACTION_STREAM_KEY, err
                ))
            })
    }

    /// Pulls up to `count` new actions for this consumer from the stream.
    /// Returns a vector of tuples containing (message_id, QueuedAction).
    async fn pull_actions(&self, count: usize) -> QueueResult<Vec<(String, QueuedAction)>> {
        if count == 0 {
            return Ok(Vec::new()); // No need to call redis if count is zero
        }
        // Delegate to the batch helper
        self.pull_actions_batch(count).await
    }

    /// Pulls ALL available new actions for this consumer from the stream.
    ///
    /// It repeatedly calls XREADGROUP in batches using the ">" ID until no
    /// new messages are returned for the target stream. This drains the
    /// currently available messages for this consumer.
    ///
    /// Returns a Vec containing tuples of (message_id, QueuedAction).
    /// The message_id is needed for later acknowledgement (`XACK`).
    async fn pull_all_available_actions(&self) -> QueueResult<Vec<(String, QueuedAction)>> {
        let mut all_actions = Vec::new();
        let mut total_fetched = 0;

        loop {
            // Fetch a batch of actions using the helper
            match self.pull_actions_batch(BATCH_PULL_SIZE).await {
                Ok(batch) => {
                    let batch_size = batch.len();
                    total_fetched += batch_size;

                    if batch.is_empty() {
                        // No more messages available in this pull cycle
                        tracing::debug!(
                             total_fetched,
                             stream = ACTION_STREAM_KEY,
                             group = ACTION_CONSUMER_GROUP,
                             consumer = %self.instance_id,
                            "Finished pulling all available actions."
                        );
                        break; // Exit the loop
                    } else {
                        tracing::trace!(batch_size, total_fetched, "Pulled batch of actions.");
                        // Extend the main vector with the results from the batch
                        all_actions.extend(batch);

                        // Optimization: If the batch was smaller than requested, we are likely at the end.
                        if batch_size < BATCH_PULL_SIZE {
                            tracing::debug!(
                                batch_size,
                                batch_pull_size = BATCH_PULL_SIZE,
                                total_fetched,
                                "Pulled partial batch, likely end of stream for now. Finishing."
                            );
                            break;
                        }
                        // Otherwise (batch was full), loop again immediately to fetch the next batch
                    }
                }
                Err(e) => {
                    // Log the error and stop pulling more actions for this cycle.
                    tracing::error!(error = %e, total_fetched, "Error pulling action batch. Aborting pull cycle.");
                    // Return the error, potentially with partially fetched actions if desired,
                    // but typically safer to return the error directly.
                    return Err(e);
                }
            }
        }

        Ok(all_actions)
    }

    /// Acknowledges processed messages using XACK.
    async fn acknowledge_actions(&self, ids: &[&str]) -> QueueResult<()> {
        if ids.is_empty() {
            return Ok(()); // Nothing to acknowledge
        }

        let mut conn = self.conn.clone();

        // Execute XACK
        let ack_count: i64 = conn
            .xack(ACTION_STREAM_KEY, ACTION_CONSUMER_GROUP, &ids) // Pass slice of &str
            .await
            .map_err(|err| QueueError::ConnectionError(format!("XACK failed: {}", err)))?;

        tracing::debug!(
            acked_count = ack_count,
            expected_count = ids.len(),
            stream = ACTION_STREAM_KEY,
            group = ACTION_CONSUMER_GROUP,
            "Acknowledged actions in Redis Stream."
        );

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
