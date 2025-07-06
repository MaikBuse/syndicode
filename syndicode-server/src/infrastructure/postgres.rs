pub mod economy;
pub mod game_tick;
pub mod init;
pub mod migration;
pub mod unit;
pub mod uow;
pub mod user;
pub mod user_verify;

use std::sync::Arc;

use crate::config::ServerConfig;
use sqlx::{pool::PoolOptions, PgPool};

// The SRID (Spatial Reference ID). 4326 is the standard for GPS (WGS 84).
pub(super) const SRID: i32 = 4326;

#[derive(Debug)]
pub struct PostgresDatabase {
    pub pool: PgPool,
}

impl PostgresDatabase {
    pub async fn new(config: Arc<ServerConfig>) -> anyhow::Result<Self> {
        tracing::info!("Initializing postgres database connection");

        let db_url = build_postgres_db_url(config.clone());

        let pool = PoolOptions::new()
            .max_connections(config.postgres.max_connections)
            .connect(&db_url)
            .await
            .map_err(|err| anyhow::format_err!(err))?;

        Ok(Self { pool })
    }
}

pub(super) fn build_postgres_db_url(config: Arc<ServerConfig>) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        urlencoding::encode(config.postgres.user.as_str()),
        urlencoding::encode(config.postgres.password.as_str()),
        config.postgres.host,
        config.postgres.port,
        config.postgres.database
    )
}
