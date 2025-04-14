pub mod leader;
pub mod limiter;
pub mod queue;

use crate::{application::ports::limiter::LimiterCategory, utils::read_env_var};
use anyhow::Context;
use queue::{ACTION_CONSUMER_GROUP, ACTION_STREAM_KEY};
use redis::{aio::MultiplexedConnection, AsyncCommands, Script};

#[derive(Clone)]
pub struct ValkeyStore {
    /// A unique identifier for this specific instance trying to acquire the lock.
    /// Used to ensure only the lock holder can release/refresh it.
    instance_id: String,
    conn: MultiplexedConnection,
    leader_config: LeaderElectionConfig,
    limiter_config: LimiterConfig,
}

impl ValkeyStore {
    pub async fn new(
        instance_id: String,
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

        ValkeyStore::ensure_consumer_group_exists(
            conn.clone(),
            ACTION_STREAM_KEY,
            ACTION_CONSUMER_GROUP,
        )
        .await?;

        Ok(Self {
            instance_id,
            conn,
            limiter_config,
            leader_config,
        })
    }
    async fn ensure_consumer_group_exists(
        mut conn: MultiplexedConnection,
        stream_key: &str,
        group_name: &str,
    ) -> anyhow::Result<()> {
        // Attempt to create the group, starting from new messages ($)
        // MKSTREAM creates the stream if it doesn't exist
        let result: Result<(), redis::RedisError> = conn
            .xgroup_create_mkstream(stream_key, group_name, "$")
            .await;

        match result {
            Ok(()) => {
                tracing::info!(
                    stream = stream_key,
                    group = group_name,
                    "Successfully created Redis consumer group."
                );
                Ok(())
            }
            Err(err) => {
                // Check if the error is specifically "BUSYGROUP Consumer Group name already exists"
                if err.to_string().contains("BUSYGROUP") {
                    tracing::info!(
                        stream = stream_key,
                        group = group_name,
                        "Redis consumer group already exists."
                    );
                    Ok(()) // It's okay if it already exists
                } else {
                    // Different error, propagate it
                    tracing::error!(stream = stream_key, group = group_name, error = %err, "Failed to create or verify Redis consumer group.");
                    Err(err.into())
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct LeaderElectionConfig {
    pub leader_lock_ttl: usize,
    // Lua scripts for safe release and refresh
    pub release_script: Script,
    pub refresh_script: Script,
}

impl LeaderElectionConfig {
    /// Creates a default configuration with a unique instance ID.
    pub fn new(leader_lock_ttl: usize) -> Self {
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
            leader_lock_ttl,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LimiterConfig {
    pub middleware_max_req: usize,
    pub middleware_window_secs: usize,
    pub game_stream_max_req: usize,
    pub game_stream_window_secs: usize,
    pub auth_max_req: usize,
    pub auth_window_secs: usize,
    pub admin_max_req: usize,
    pub admin_window_secs: usize,
}

impl LimiterConfig {
    pub fn get_max_requests(&self, category: LimiterCategory) -> usize {
        match category {
            LimiterCategory::Middleware => self.middleware_max_req,
            LimiterCategory::GameStream => self.game_stream_max_req,
            LimiterCategory::Auth => self.auth_max_req,
            LimiterCategory::Admin => self.admin_max_req,
        }
    }

    pub fn get_window_secs(&self, category: LimiterCategory) -> usize {
        match category {
            LimiterCategory::Middleware => self.middleware_window_secs,
            LimiterCategory::GameStream => self.game_stream_window_secs,
            LimiterCategory::Auth => self.auth_window_secs,
            LimiterCategory::Admin => self.admin_window_secs,
        }
    }
}
