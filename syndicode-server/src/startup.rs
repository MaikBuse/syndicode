mod bootstrap;
mod leader;
mod logging;
mod server;
mod services;

use crate::{
    config::Config,
    infrastructure::{
        postgres::PostgresDatabase,
        valkey::{LeaderElectionConfig, LimiterConfig, ValkeyStore},
    },
};
use leader::run_leader_election_loop;
use services::AppState;
use std::sync::Arc;

const LIMIT_MAX_REQUESTS: usize = 100;
const LIMIT_WINDOW: usize = 60;

pub async fn start_server() -> anyhow::Result<()> {
    let config = Arc::new(Config::new()?);

    logging::init();

    let pg_pool = Arc::new(PostgresDatabase::init().await?);

    let valkey_store = Arc::new(
        ValkeyStore::new(
            LeaderElectionConfig::new(config.instance_id.clone(), config.leader_lock_ttl),
            LimiterConfig::new(LIMIT_MAX_REQUESTS, LIMIT_WINDOW),
        )
        .await?,
    );

    let state =
        AppState::build_services(config.clone(), pg_pool.clone(), valkey_store.clone()).await?;

    bootstrap::run(pg_pool, state.bootstrap_admin_uc.clone()).await?;

    tokio::spawn(run_leader_election_loop(
        state.leader_elector.clone(),
        config.instance_id.clone(),
        config.leader_lock_refresh_interval,
        config.non_leader_retry_acquisition_internal,
    ));

    server::start_grpc_services(config, state, valkey_store.clone()).await?;

    Ok(())
}
