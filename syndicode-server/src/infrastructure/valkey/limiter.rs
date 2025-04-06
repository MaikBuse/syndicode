use super::ValkeyStore;
use crate::application::ports::limiter::{LimitationError, LimitationResult, RateLimitEnforcer};
use redis::AsyncCommands;
use std::time::{SystemTime, UNIX_EPOCH};

#[tonic::async_trait]
impl RateLimitEnforcer for ValkeyStore {
    async fn check(&self, ip_address: &str) -> LimitationResult<()> {
        let mut conn = self.conn.clone();

        let key = format!("syndicode:rate_limit:{}", ip_address);
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH")
            .as_millis();
        let now_ms_str = now_ms.to_string(); // Store string representation

        let window_millis = (self.limiter_config.window_secs as u128) * 1000;
        let window_start = now_ms.saturating_sub(window_millis);

        // Atomically clean old entries AND add the current timestamp
        // We add the timestamp optimistically.
        // It's slightly better to remove old entries *first*.
        let pipe_result: Result<((), (), ()), redis::RedisError> = redis::pipe() // Expect 3 results () from zrembyscore, zadd, expire
            .atomic()
            .zrembyscore(&key, "-inf", window_start as f64) // Remove outdated entries first
            .zadd(&key, &now_ms_str, now_ms as f64) // Add current request timestamp
            .expire(&key, self.limiter_config.window_secs as i64 * 2) // Refresh expiration
            .query_async(&mut conn)
            .await;

        pipe_result.map_err(|e| {
            tracing::error!(error = ?e, key = %key, "Redis pipeline command failed during rate limit check (add/clean step)");
            LimitationError::Internal(anyhow::Error::from(e).context("Redis pipeline failed (add/clean step)").to_string())
        })?;

        // Check the count *after* adding the current timestamp
        let count_result: Result<usize, redis::RedisError> = conn
            .zcount(&key, window_start as f64, now_ms as f64) // Count includes the one we just added
            .await;

        let count = count_result.map_err(|e| {
             tracing::error!(error = ?e, key = %key, "Redis zcount command failed during rate limit check");
             LimitationError::Internal(anyhow::Error::from(e).context("Redis zcount failed").to_string())
        })?;

        // Decide if the limit was exceeded
        // If the count *after* adding is strictly greater than max_requests,
        // it means this request pushed it over the limit (or it was already over).
        // In this case, we deny the request and undo the addition.
        if count > self.limiter_config.max_requests {
            // Use '>' because we already added the current request
            tracing::warn!(
                %ip_address,
                %key,
                current_count_after_add = count,
                max_requests = self.limiter_config.max_requests,
                "Rate limit exceeded, removing added timestamp"
            );

            // Remove the timestamp we just added
            // This is the crucial part to meet the requirement.
            let remove_result: Result<usize, redis::RedisError> =
                conn // ZREM returns number of elements removed
                    .zrem(&key, &now_ms_str)
                    .await;

            match remove_result {
                Ok(removed_count) => {
                    if removed_count == 0 {
                        // This shouldn't happen often if the pipeline succeeded, but log it.
                        tracing::warn!(key = %key, member = %now_ms_str, "Tried to remove rate limit timestamp after exceeding limit, but it was already gone.");
                    }
                }
                Err(e) => {
                    // Log the error, but still return RateExhausted as the user *is* limited.
                    // The counter in Redis might be slightly off now, but denying the request is the primary goal.
                    tracing::error!(error = ?e, key = %key, member = %now_ms_str, "Failed to remove rate limit timestamp after exceeding limit");
                }
            }

            Err(LimitationError::RateExhausted)
        } else {
            // If count <= max_requests, the request is allowed, and the timestamp we added is kept.
            Ok(())
        }
    }
}
