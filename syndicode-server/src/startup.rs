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
    presentation::{broadcaster::GameTickBroadcaster, game::UserChannels},
};
use dashmap::DashMap;
use services::AppState;
use std::{sync::Arc, time::Duration};

pub async fn start_server() -> anyhow::Result<()> {
    let config = Arc::new(Config::new()?);

    logging::init();

    let user_channels: UserChannels = Arc::new(DashMap::new());

    let pg_pool = Arc::new(PostgresDatabase::init().await?);

    let valkey_store = Arc::new(
        ValkeyStore::new(
            config.instance_id.clone(),
            LeaderElectionConfig::new(config.leader_lock_ttl),
            LimiterConfig {
                middleware_max_req: 150,
                middleware_window_secs: 60,
                game_stream_max_req: 100,
                game_stream_window_secs: 10,
                auth_max_req: 5,
                auth_window_secs: 60,
                admin_max_req: 10,
                admin_window_secs: 60,
            },
        )
        .await?,
    );

    let state = AppState::build_services(
        config.clone(),
        pg_pool.clone(),
        valkey_store.clone(),
        user_channels.clone(),
    )
    .await?;

    // Bootstrap
    bootstrap::run()
        .pool(pg_pool)
        .bootstrap_admin_uc(state.bootstrap_admin_uc.clone())
        .bootstrap_economy_uc(state.bootstrap_economy_uc.clone())
        .call()
        .await?;

    // Spawn leader loop
    let leader_loop_manager = LeaderLoopManager::new(
        state.leader_elector.clone(),
        state.game_tick_processor.clone(),
        config.instance_id.clone(),
        config.leader_lock_refresh_interval,
        config.non_leader_acquisition_retry_internal,
        Duration::from_millis(config.game_tick_interval as u64),
    );

    tokio::spawn(leader_loop_manager.run());

    // Grpc Server
    server::start_grpc_services(config, state, valkey_store.clone()).await?;

    // Game Tick Notification Broadcaster
    let broadcaster = GameTickBroadcaster::builder()
        .valkey_client(valkey_store.get_client())
        .user_channels(user_channels.clone())
        .build();
    broadcaster.spawn_listen_and_broadcast_task();

    Ok(())
}
