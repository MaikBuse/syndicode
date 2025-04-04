use std::sync::Arc;

use crate::infrastructure::postgres::PostgresDatabase;

mod bootstrap;
mod logging;
mod server;
mod services;

pub async fn start_server() -> anyhow::Result<()> {
    logging::init();

    let pool = Arc::new(PostgresDatabase::init().await?);
    let state = services::build_services(pool.clone()).await;

    bootstrap::run(pool, state.create_user_uc.clone()).await?;

    tokio::spawn(engine_loop(state.engine.clone()));

    server::start_grpc_services(state).await?;

    Ok(())
}
