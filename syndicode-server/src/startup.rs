mod bootstrap;
mod logging;
mod server;
mod services;

use crate::{
    application::leader::LeaderLoopManager,
    config::Config,
    infrastructure::{
        postgres::PostgresDatabase,
        valkey::{LeaderElectionConfig, LimiterConfig, ValkeyStore},
    },
};
use services::AppState;
use std::{sync::Arc, time::Duration};

const LIMIT_MAX_REQUESTS: usize = 100;
const LIMIT_WINDOW: usize = 60;

pub async fn start_server() -> anyhow::Result<()> {
    let config = Arc::new(Config::new()?);

    logging::init();

    let pg_pool = Arc::new(PostgresDatabase::init().await?);

    let valkey_store = Arc::new(
        ValkeyStore::new(
            config.instance_id.clone(),
            LeaderElectionConfig::new(config.leader_lock_ttl),
            LimiterConfig::new(LIMIT_MAX_REQUESTS, LIMIT_WINDOW),
        )
        .await?,
    );

    let state =
        AppState::build_services(config.clone(), pg_pool.clone(), valkey_store.clone()).await?;

    bootstrap::run(pg_pool, state.bootstrap_admin_uc.clone()).await?;

    let leader_loop_manager = LeaderLoopManager::new(
        state.leader_elector.clone(),
        state.game_tick_processor.clone(),
        config.instance_id.clone(),
        config.leader_lock_refresh_interval,
        config.non_leader_acquisition_retry_internal,
        Duration::from_millis(config.game_tick_interval as u64),
    );

    tokio::spawn(leader_loop_manager.run());

    server::start_grpc_services(config, state, valkey_store.clone()).await?;

    Ok(())
}
