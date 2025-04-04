mod bootstrap;
mod engine;
mod logging;
mod server;
mod services;

use crate::{
    config::Config,
    infrastructure::{postgres::PostgresDatabase, valkey::ValkeyStore},
};
use engine::engine_loop;
use std::sync::Arc;

const LIMIT_MAX_REQUESTS: usize = 100;
const LIMIT_WINDOW: usize = 60;

pub async fn start_server() -> anyhow::Result<()> {
    let config = Arc::new(Config::new()?);

    logging::init();

    let pg_pool = Arc::new(PostgresDatabase::init().await?);

    let valkey_store = Arc::new(ValkeyStore::new(LIMIT_MAX_REQUESTS, LIMIT_WINDOW).await?);

    let state = services::build_services(pg_pool.clone(), valkey_store.clone()).await?;

    bootstrap::run(pg_pool, state.bootstrap_admin_uc.clone()).await?;

    tokio::spawn(engine_loop(state.engine.clone()));

    server::start_grpc_services(config, state, valkey_store.clone()).await?;

    Ok(())
}
