pub mod leader;
pub mod limiter;
pub mod outcome;
pub mod puller;
pub mod queuer;

use std::sync::Arc;

use crate::config::ServerConfig;
use anyhow::Context;
use redis::{aio::MultiplexedConnection, AsyncCommands, Script};

pub const ACTION_STREAM_KEY: &str = "syndicode:game_actions";
pub const ACTION_CONSUMER_GROUP: &str = "leader_processors";
pub const PAYLOAD_FIELD: &str = "payload";

#[derive(Clone)]
pub struct ValkeyStore {
    config: Arc<ServerConfig>,
    client: redis::Client,
    conn: MultiplexedConnection,
    leader_scripts: LeaderElectionScripts,
}

impl ValkeyStore {
    pub async fn new(config: Arc<ServerConfig>) -> anyhow::Result<Self> {
        let conn_string = match config.valkey.password.is_empty() {
            true => {
                format!(
                    "redis://:{}@{}:6379",
                    urlencoding::encode(config.valkey.password.as_str()),
                    config.valkey.host,
                )
            }
            false => {
                format!("redis://{}:6379", config.valkey.host)
            }
        };

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
            config,
            client,
            conn,
            leader_scripts: LeaderElectionScripts::default(),
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

    pub fn get_client(&self) -> redis::Client {
        self.client.clone()
    }
}

#[derive(Clone, Debug)]
pub struct LeaderElectionScripts {
    // Lua scripts for safe release and refresh
    pub release_script: Script,
    pub refresh_script: Script,
}

impl Default for LeaderElectionScripts {
    fn default() -> Self {
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
        }
    }
}
