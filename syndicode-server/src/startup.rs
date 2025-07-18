mod logging;
mod provider;
mod server;

use crate::{
    application::leader::LeaderLoopManager,
    cli::Cli,
    config::ServerConfig,
    infrastructure::{postgres::PostgresDatabase, valkey::ValkeyStore},
    presentation::{broadcaster::GameTickBroadcaster, game::user_channel_guard::UserChannels},
};
use clap::Parser;
use dashmap::DashMap;
use provider::AppProvider;
use std::{sync::Arc, time::Duration};

pub async fn start_server() -> anyhow::Result<()> {
    let cli = Arc::new(Cli::parse());

    let config = Arc::new(ServerConfig::new()?);

    logging::init();

    let user_channels: UserChannels = Arc::new(DashMap::new());

    let pg_db = Arc::new(PostgresDatabase::new(config.clone()).await?);

    let valkey_store = Arc::new(ValkeyStore::new(config.clone()).await?);

    let provider = AppProvider::build_services()
        .config(config.clone())
        .cli(cli)
        .pg_db(pg_db.clone())
        .valkey(valkey_store.clone())
        .user_channels(user_channels.clone())
        .call()
        .await?;

    // Spawn leader loop
    let leader_loop_manager = LeaderLoopManager::builder()
        .leader_elector(provider.leader_elector.clone())
        .game_tick_processor(provider.game_tick_processor.clone())
        .instance_id(config.general.instance_id.clone())
        .leader_lock_refresh_interval(Duration::from_millis(
            config.processor.leader_lock_refresh_interval as u64,
        ))
        .non_leader_acquisition_retry_interval(Duration::from_millis(
            config.processor.non_leader_acquisition_retry_internal as u64,
        ))
        .game_tick_interval(Duration::from_millis(
            config.processor.game_tick_interval as u64,
        ))
        .initialization_orchestrator(provider.initialization_orchestrator.clone())
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
