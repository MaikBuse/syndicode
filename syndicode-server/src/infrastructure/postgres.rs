pub mod corporation;
pub mod migration;
pub mod unit;
pub mod uow;
pub mod user;

use sqlx::{pool::PoolOptions, PgPool};
use std::env;

const MAX_CONNECTIONS: u32 = 5;

#[derive(Debug)]
pub struct PostgresDatabase;

impl PostgresDatabase {
    pub async fn init() -> sqlx::Result<PgPool> {
        tracing::info!("Initializing postgres database connection");

        let postgres_user =
            env::var("POSTGRES_USER").expect("Environment variable 'POSTGRES_USER' must be set");
        let postgres_password = env::var("POSTGRES_PASSWORD")
            .expect("Environment variable 'POSTGRES_PASSWORD' must be set");
        let postgres_host =
            env::var("POSTGRES_HOST").expect("Environment variable 'POSTGRES_HOST' must be set");
        let postgres_port =
            env::var("POSTGRES_PORT").expect("Environment variable 'POSTGRES_PORT' must be set");
        let postgres_db =
            env::var("POSTGRES_DB").expect("Environment variable 'POSTGRES_DB' must be set");

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
    }
}
