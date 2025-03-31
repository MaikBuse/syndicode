mod application;
mod domain;
mod engine;
mod infrastructure;
mod presentation;

use application::admin::create_user::CreateUserUseCase;
use application::admin::delete_user::DeleteUserUseCase;
use application::auth::login::LoginUseCase;
use application::economy::get_corporation::GetCorporationUseCase;
use application::warfare::list_units::ListUnitsUseCase;
use application::warfare::spawn_unit::SpawnUnitUseCase;
use dashmap::DashMap;
use engine::Engine;
use infrastructure::crypto::CryptoService;
use infrastructure::postgres::PostgresDatabase;
use presentation::admin::AdminPresenter;
use presentation::auth::AuthPresenter;
use presentation::game::GamePresenter;
use presentation::middleware::MiddlewareLayer;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;
use syndicode_proto::syndicode_interface_v1::admin_service_server::AdminServiceServer;
use syndicode_proto::syndicode_interface_v1::auth_service_server::AuthServiceServer;
use syndicode_proto::syndicode_interface_v1::game_service_server::GameServiceServer;
use tokio::sync::Mutex;
use tokio::time::{self, Instant};
use tonic::transport::Server;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

pub const SOCKET_ADDR: &str = "[::]:50051";

const JOB_INTERVAL: Duration = Duration::from_secs(1);

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
        .set_serving::<AdminServiceServer<AdminPresenter>>()
        .await;

    // Setup database
    let db = Arc::new(PostgresDatabase::init(admin_password).await?);

    // Setup crypto service
    let crypto = Arc::new(CryptoService::new(jwt_secret));

    // Auth use cases
    let login_uc = Arc::new(LoginUseCase::new(Arc::clone(&db), Arc::clone(&crypto)));

    // Admin use cases
    let create_user_uc = Arc::new(CreateUserUseCase::new(Arc::clone(&db), Arc::clone(&crypto)));
    let delete_user_uc = Arc::new(DeleteUserUseCase::new(Arc::clone(&db)));

    // Warfare use cases
    let spawn_unit_uc = Arc::new(SpawnUnitUseCase::new(Arc::clone(&db)));
    let list_units_uc = Arc::new(ListUnitsUseCase::new(Arc::clone(&db)));

    // Economy use cases
    let get_corporation_uc = Arc::new(GetCorporationUseCase::new(Arc::clone(&db)));

    // Setup tick-engine
    let jobs = Arc::new(Mutex::new(VecDeque::new()));
    let user_channels = Arc::new(DashMap::new());
    let engine = Arc::new(Mutex::new(Engine::init(
        Arc::clone(&jobs),
        Arc::clone(&spawn_unit_uc),
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

    let game_presenter = GamePresenter {
        jobs: Arc::clone(&jobs),
        user_channels: Arc::clone(&user_channels),
        list_units_uc: Arc::clone(&list_units_uc),
        get_corporation_uc: Arc::clone(&get_corporation_uc),
    };

    let admin_presenter = AdminPresenter {
        create_user_uc: Arc::clone(&create_user_uc),
        delete_user_uc: Arc::clone(&delete_user_uc),
    };

    let auth_presenter = AuthPresenter {
        create_user_uc: Arc::clone(&create_user_uc),
        login_uc: Arc::clone(&login_uc),
    };

    tracing::info!("Starting server...");

    Server::builder()
        .layer(MiddlewareLayer::new(Arc::clone(&crypto)))
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(GameServiceServer::new(game_presenter))
        .add_service(AdminServiceServer::new(admin_presenter))
        .add_service(AuthServiceServer::new(auth_presenter))
        .serve(addr)
        .await?;

    Ok(())
}
