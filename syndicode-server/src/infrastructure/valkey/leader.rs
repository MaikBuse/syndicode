use super::ValkeyStore;
use crate::application::ports::leader::{LeaderElectionError, LeaderElectionResult, LeaderElector};

const LOCK_KEY: &str = "syndicode:leader_lock";

#[tonic::async_trait]
impl LeaderElector for ValkeyStore {
    /// Attempts to acquire the lock using `SET key instance_id NX PX ttl`.
    async fn try_acquire(&self) -> LeaderElectionResult<bool> {
        let mut conn = self.conn.clone();

        let value = &self.leader_config.instance_id;

        // Type annotation ::<Option<String>> is important here.
        // SET ... NX returns "OK" (as a String) on success, or nil (None) if not set.
        let result: Option<String> = redis::cmd("SET")
            .arg(LOCK_KEY)
            .arg(value)
            .arg("NX") // Only set if key does not exist
            .arg("PX") // Set expiry in milliseconds
            .arg(self.leader_config.leader_lock_ttl)
            .query_async(&mut conn)
            .await?;

        match result {
            Some(ref s) if s == "OK" => Ok(true), // Successfully acquired the lock
            None => Ok(false),                    // Lock already held by someone else
            Some(_) => {
                // Should not happen with "OK" string, but handle defensively
                Err(LeaderElectionError::LockAcquireFailed {
                    key: LOCK_KEY.to_string(),
                    details: "Unexpected response from Redis SET NX command".to_string(),
                })
            }
        }
    }

    /// Releases the lock using the safe Lua script.
    async fn release(&self) -> LeaderElectionResult<()> {
        let mut conn = self.conn.clone();
        let instance_id = &self.leader_config.instance_id;

        let result: i32 = self
            .leader_config
            .release_script
            .key(LOCK_KEY) // Pass lock_key as KEYS[1]
            .arg(instance_id) // Pass instance_id as ARGV[1]
            .invoke_async(&mut conn)
            .await?;

        match result {
            1 => {
                tracing::debug!("Successfully released lock '{}'", LOCK_KEY);
                Ok(())
            }
            0 => {
                // This is not necessarily an error from the perspective of trying to release.
                // It just means we didn't hold the lock when the command ran.
                tracing::warn!(
                    "Could not release lock '{}': Not held by this instance ('{}') or expired.",
                    LOCK_KEY,
                    instance_id
                );
                // We might choose to return Ok(()) here, as the goal (lock is released by us) is achieved,
                // or return NotHoldingLock if the caller needs to know specifically. Let's return Ok for simplicity now.
                Ok(())
                // Err(LeaderElectionError::NotHoldingLock { key: key.clone(), instance_id: instance_id.clone() })
            }
            _ => Err(LeaderElectionError::LockReleaseFailed {
                key: LOCK_KEY.to_string(),
                details: format!("Unexpected response from release script: {}", result),
            }),
        }
    }

    /// Refreshes the lock TTL using the safe Lua script.
    async fn refresh(&self) -> LeaderElectionResult<()> {
        let mut conn = self.conn.clone();
        let instance_id = &self.leader_config.instance_id;

        let result: i32 = self
            .leader_config
            .refresh_script
            .key(LOCK_KEY) // KEYS[1]
            .arg(instance_id) // ARGV[1]
            .arg(self.leader_config.leader_lock_ttl) // ARGV[2]
            .invoke_async(&mut conn)
            .await?;

        match result {
            1 => {
                tracing::trace!(
                    "Successfully refreshed lock '{}' for instance '{}'",
                    LOCK_KEY,
                    instance_id
                );
                Ok(())
            }
            0 => {
                // Explicitly return an error here, as refresh implies we expect to hold the lock.
                tracing::warn!(
                    "Could not refresh lock '{}': Not held by this instance ('{}') or expired.",
                    LOCK_KEY,
                    instance_id
                );
                Err(LeaderElectionError::NotHoldingLock {
                    key: LOCK_KEY.to_string(),
                    instance_id: instance_id.clone(),
                })
            }
            _ => Err(LeaderElectionError::LockRefreshFailed {
                key: LOCK_KEY.to_string(),
                details: format!("Unexpected response from refresh script: {}", result),
            }),
        }
    }
}
