mod domain;
mod engine;
mod infrastructure;
mod presentation;
mod service;

use dashmap::DashMap;
use engine::Engine;
use governor::{Quota, RateLimiter};
use infrastructure::postgres::PostgresDatabase;
use presentation::control::ControlPresenter;
use presentation::middleware::MiddlewareLayer;
use service::control::ControlService;
use service::economy::EconomyService;
use service::warfare::WarfareService;
use std::collections::VecDeque;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use syndicode_proto::control::control_server::ControlServer;
use tokio::sync::Mutex;
use tokio::time::{self, Instant};
use tonic::transport::Server;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

pub const SOCKET_ADDR: &str = "[::]:50051";

const JOB_INTERVAL: Duration = Duration::from_secs(1);
const REQUESTS_PER_SECOND: u32 = 5;

const JWT_SECRET_ENV: &str = "JWT_SECRET";
pub const ADMIN_PASSWORD_ENV: &str = "ADMIN_PASSWORD";

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env()) // reads RUST_LOG env var
        .with(fmt::layer().pretty()) // use .json() instead of .pretty() for JSON logs
        .init();

    // Check environment variables
    let jwt_secret =
        std::env::var(JWT_SECRET_ENV).expect("Environment variable 'JWT_SECRET' must be set");
    let admin_password = std::env::var(ADMIN_PASSWORD_ENV)
        .expect("Environment variable 'ADMIN_PASSWORD' must be set");

    // Add health checks for servers
    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<ControlServer<ControlPresenter>>()
        .await;

    // Setup database
    let database = Arc::new(PostgresDatabase::init(admin_password).await?);
    let db_control_clone = Arc::clone(&database);
    let db_economy_clone = Arc::clone(&database);
    let db_warfare_clone = Arc::clone(&database);

    // Init rate limiter
    let quota = Quota::per_second(NonZeroU32::new(REQUESTS_PER_SECOND).unwrap());
    let user_limiter = Arc::new(RateLimiter::keyed(quota));

    // Setup services
    let control_service = Arc::new(ControlService::new(db_control_clone, jwt_secret.clone()));
    let economy_service = Arc::new(EconomyService::new(db_economy_clone));
    let warfare_service = Arc::new(WarfareService::new(db_warfare_clone));

    // Setup tick-engine
    let jobs = Arc::new(Mutex::new(VecDeque::new()));
    let user_channels = Arc::new(DashMap::new());
    let engine = Arc::new(Mutex::new(Engine::init(
        Arc::clone(&jobs),
        Arc::clone(&control_service),
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

    // Setup reflection service for service discovery
    let reflection_service = syndicode_proto::create_reflection_service()?;

    let addr = SOCKET_ADDR.parse()?;

    let control_presenter = ControlPresenter {
        jobs: Arc::clone(&jobs),
        user_limiter: Arc::clone(&user_limiter),
        user_channels: Arc::clone(&user_channels),
        control_service: Arc::clone(&control_service),
        economy_service: Arc::clone(&economy_service),
        warfare_service: Arc::clone(&warfare_service),
    };

    tracing::info!("Starting server...");

    Server::builder()
        .layer(MiddlewareLayer::new(jwt_secret, user_limiter))
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(ControlServer::new(control_presenter))
        .serve(addr)
        .await?;

    Ok(())
}
