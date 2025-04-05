pub mod limiter;
pub mod queue;

use crate::utils::read_env_var;
use anyhow::Context;
use redis::aio::MultiplexedConnection;

#[derive(Clone)]
pub struct ValkeyStore {
    conn: MultiplexedConnection,
    max_requests: usize,
    window_secs: usize,
}

impl ValkeyStore {
    pub async fn new(max_requests: usize, window_secs: usize) -> anyhow::Result<Self> {
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
            max_requests,
            window_secs,
        })
    }
}
