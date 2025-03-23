mod domain;
mod engine;
mod infrastructure;
mod presentation;
mod service;

use dashmap::DashMap;
use engine::Engine;
use infrastructure::sqlite::SqliteDatabase;
use presentation::control::ControlPresenter;
use presentation::middleware::JwtAuthLayer;
use presentation::proto::control::control_server::ControlServer;
use service::control::ControlService;
use service::economy::EconomyService;
use service::warfare::WarfareService;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{self, Instant};
use tonic::transport::Server;

mod reflection {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("reflection_descriptor");
}

const JOB_INTERVAL: Duration = Duration::from_secs(1);

const JWT_SECRET_ENV: &str = "JWT_SECRET";
const ADMIN_PASSWORD: &str = "ADMIN_PASSWORD";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let jwt_secret =
        std::env::var(JWT_SECRET_ENV).expect("Environment variable 'JWT_SECRET' must be set");
    let admin_password =
        std::env::var(ADMIN_PASSWORD).expect("Environment variable 'ADMIN_PASSWORD' must be set");

    let sqlite_database = Arc::new(Mutex::new(SqliteDatabase::init(admin_password).await?));
    let sqlite_db_control_clone_1 = Arc::clone(&sqlite_database);
    let sqlite_db_control_clone_2 = Arc::clone(&sqlite_database);
    let sqlite_db_economy_clone_1 = Arc::clone(&sqlite_database);
    let sqlite_db_economy_clone_2 = Arc::clone(&sqlite_database);
    let sqlite_db_warfare_clone_1 = Arc::clone(&sqlite_database);
    let sqlite_db_warfare_clone_2 = Arc::clone(&sqlite_database);

    let control_service = Arc::new(ControlService::new(
        sqlite_db_control_clone_1,
        sqlite_db_control_clone_2,
        jwt_secret.clone(),
    ));
    let economy_service = Arc::new(EconomyService::new(
        sqlite_db_economy_clone_1,
        sqlite_db_economy_clone_2,
    ));
    let warfare_service = Arc::new(WarfareService::new(
        sqlite_db_warfare_clone_1,
        sqlite_db_warfare_clone_2,
    ));

    let jobs = Arc::new(DashMap::new());
    let user_channels = Arc::new(DashMap::new());
    let engine = Arc::new(Mutex::new(Engine::init(
        Arc::clone(&jobs),
        Arc::clone(&control_service),
        Arc::clone(&economy_service),
        Arc::clone(&warfare_service),
    )));

    // Spawn the background job
    tokio::spawn(async move {
        let mut interval = time::interval(JOB_INTERVAL);
        loop {
            let next_tick = Instant::now() + JOB_INTERVAL;

            let engine_clone = Arc::clone(&engine);

            // Spawn job without waiting
            tokio::spawn(async move {
                {
                    let mut engine = engine_clone.lock().await;
                    if let Err(err) = engine.advance_epoch().await {
                        tracing::error!("{}", err);
                    }
                }
            });

            // Wait until exactly one second from the last tick
            interval.tick().await;
            tokio::time::sleep_until(next_tick).await;
        }
    });

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(reflection::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    let addr = "[::1]:50051".parse()?;

    let control_presenter = ControlPresenter {
        jobs: Arc::clone(&jobs),
        user_channels: Arc::clone(&user_channels),
        control_service: Arc::clone(&control_service),
        economy_service: Arc::clone(&economy_service),
        warfare_service: Arc::clone(&warfare_service),
    };

    Server::builder()
        .layer(JwtAuthLayer::new(jwt_secret))
        .add_service(reflection_service)
        .add_service(ControlServer::new(control_presenter))
        .serve(addr)
        .await?;

    Ok(())
}
