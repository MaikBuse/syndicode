mod logging;
mod provider;
mod server;

use crate::{
    application::leader::LeaderLoopManager,
    config::ServerConfig,
    infrastructure::{postgres::PostgresDatabase, valkey::ValkeyStore},
    presentation::{broadcaster::GameTickBroadcaster, game::user_channel_guard::UserChannels},
};
use dashmap::DashMap;
use provider::AppProvider;
use std::{sync::Arc, time::Duration};

pub async fn start_server() -> anyhow::Result<()> {
    let config = Arc::new(ServerConfig::new()?);

    logging::init();

    let user_channels: UserChannels = Arc::new(DashMap::new());

    let pg_db = Arc::new(PostgresDatabase::new(config.clone()).await?);

    let valkey_store = Arc::new(ValkeyStore::new(config.clone()).await?);

    let provider = AppProvider::build_services(
        config.clone(),
        pg_db.clone(),
        valkey_store.clone(),
        user_channels.clone(),
    )
    .await?;

    // Bootstrap
    provider.bootstrap_orchestrator.run().await?;

    // Spawn leader loop
    let leader_loop_manager = LeaderLoopManager::builder()
        .leader_elector(provider.leader_elector.clone())
        .game_tick_processor(provider.game_tick_processor.clone())
        .instance_id(config.general.instance_id.clone())
        .leader_lock_refresh_interval(config.processor.leader_lock_refresh_interval)
        .non_leader_acquisition_retry_interval(
            config.processor.non_leader_acquisition_retry_internal,
        )
        .game_tick_interval(Duration::from_millis(
            config.processor.game_tick_interval as u64,
        ))
        .build();

    tokio::spawn(leader_loop_manager.run());

    // Game Tick Notification Broadcaster
    let broadcaster = GameTickBroadcaster::builder()
        .valkey_client(valkey_store.get_client())
        .user_channels(user_channels.clone())
        .build();
    broadcaster.spawn_listen_and_broadcast_task();

    // Grpc Server
    server::start_grpc_services()
        .config(config.clone())
        .app(provider)
        .valkey(valkey_store.clone())
        .call()
        .await?;

    Ok(())
}
