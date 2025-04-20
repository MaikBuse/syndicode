pub mod corporation;
pub mod game_tick;
pub mod migration;
pub mod unit;
pub mod uow;
pub mod user;
pub mod user_verify;

use crate::utils::read_env_var;
use sqlx::{pool::PoolOptions, PgPool};

const MAX_CONNECTIONS: u32 = 5;

#[derive(Debug)]
pub struct PostgresDatabase;

impl PostgresDatabase {
    pub async fn init() -> anyhow::Result<PgPool> {
        tracing::info!("Initializing postgres database connection");

        let postgres_user = read_env_var("POSTGRES_USER")?;
        let postgres_password = read_env_var("POSTGRES_PASSWORD")?;
        let postgres_host = read_env_var("POSTGRES_HOST")?;
        let postgres_port = read_env_var("POSTGRES_PORT")?;
        let postgres_db = read_env_var("POSTGRES_DB")?;

        let conn_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            urlencoding::encode(postgres_user.as_str()),
            urlencoding::encode(postgres_password.as_str()),
            postgres_host,
            postgres_port,
            postgres_db
        );

        PoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(&conn_string)
            .await
            .map_err(|err| anyhow::format_err!(err))
    }
}
