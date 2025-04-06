pub mod leader;
pub mod limiter;
pub mod queue;

use crate::utils::read_env_var;
use anyhow::Context;
use redis::{aio::MultiplexedConnection, Script};

#[derive(Clone)]
pub struct ValkeyStore {
    conn: MultiplexedConnection,
    leader_config: LeaderElectionConfig,
    limiter_config: LimiterConfig,
}

impl ValkeyStore {
    pub async fn new(
        leader_config: LeaderElectionConfig,
        limiter_config: LimiterConfig,
    ) -> anyhow::Result<Self> {
        let valkey_host = read_env_var("VALKEY_HOST")?;
        let valkey_password = read_env_var("VALKEY_PASSWORD")?;

        let conn_string = format!(
            "redis://:{}@{}:6379",
            urlencoding::encode(valkey_password.as_str()),
            valkey_host,
        );

        let client =
            redis::Client::open(conn_string).context("Failed to parse Redis connection string")?;

        let conn = client
            .get_multiplexed_tokio_connection()
            .await
            .context("Failed to establish multiplexed Valkey connection")?;

        Ok(Self {
            conn,
            limiter_config,
            leader_config,
        })
    }
}

#[derive(Clone, Debug)]
pub struct LeaderElectionConfig {
    /// A unique identifier for this specific instance trying to acquire the lock.
    /// Used to ensure only the lock holder can release/refresh it.
    pub instance_id: String,
    pub leader_lock_ttl: usize,
    // Lua scripts for safe release and refresh
    pub release_script: Script,
    pub refresh_script: Script,
}

impl LeaderElectionConfig {
    /// Creates a default configuration with a unique instance ID.
    pub fn new(instance_id: String, leader_lock_ttl: usize) -> Self {
        // Lua script for safe release:
        // Deletes the key ONLY if its value matches the provided instance_id.
        // KEYS[1]: lock_key
        // ARGV[1]: instance_id
        // Returns 1 if deleted, 0 otherwise.
        let release_script = Script::new(
            r#"
            if redis.call("GET", KEYS[1]) == ARGV[1] then
              return redis.call("DEL", KEYS[1])
            else
              return 0
            end
            "#,
        );

        // Lua script for safe refresh:
        // Refreshes the TTL ONLY if the key's value matches the provided instance_id.
        // KEYS[1]: lock_key
        // ARGV[1]: instance_id
        // ARGV[2]: new_ttl_milliseconds
        // Returns 1 if PEXPIRE was called, 0 otherwise.
        let refresh_script = Script::new(
            r#"
            if redis.call("GET", KEYS[1]) == ARGV[1] then
              return redis.call("PEXPIRE", KEYS[1], ARGV[2])
            else
              return 0
            end
            "#,
        );

        Self {
            release_script,
            refresh_script,
            instance_id,
            leader_lock_ttl,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LimiterConfig {
    pub max_requests: usize,
    pub window_secs: usize,
}

impl LimiterConfig {
    pub fn new(max_requests: usize, window_secs: usize) -> Self {
        Self {
            max_requests,
            window_secs,
        }
    }
}
