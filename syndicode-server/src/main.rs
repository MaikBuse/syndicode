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
use domain::repository::corporation::CorporationRepository;
use domain::repository::unit::UnitRespository;
use domain::repository::user::UserRepository;
use engine::Engine;
use infrastructure::crypto::CryptoService;
use infrastructure::postgres::corporation::PgCorporationService;
use infrastructure::postgres::unit::PgUnitService;
use infrastructure::postgres::uow::PostgresUnitOfWork;
use infrastructure::postgres::user::{PgUserRepository, PgUserService};
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

const JOB_INTERVAL: Duration = Duration::from_secs(3);

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env()) // reads RUST_LOG env var
        .with(fmt::layer().pretty()) // use .json() instead of .pretty() for JSON logs
        .init();

    // Add health checks for servers
    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<AdminServiceServer<AdminPresenter<PostgresUnitOfWork>>>()
        .await;

    // Crypto service
    let crypto = Arc::new(CryptoService::new());

    // Database Pool
    let pool = Arc::new(PostgresDatabase::init().await?);

    // Unit of Work
    let uow = Arc::new(PostgresUnitOfWork::new(Arc::clone(&pool)));

    // Services
    let user_service: Arc<dyn UserRepository> =
        Arc::new(PgUserService::new(Arc::clone(&pool), PgUserRepository));
    let unit_service: Arc<dyn UnitRespository> = Arc::new(PgUnitService::new(Arc::clone(&pool)));
    let corporation_service: Arc<dyn CorporationRepository> =
        Arc::new(PgCorporationService::new(Arc::clone(&pool)));

    // Auth use cases
    let login_uc = Arc::new(LoginUseCase::new(
        Arc::clone(&crypto),
        Arc::clone(&user_service),
    ));

    // Admin use cases
    let create_user_uc = Arc::new(CreateUserUseCase::new(
        Arc::clone(&crypto),
        Arc::clone(&uow),
        Arc::clone(&user_service),
    ));
    let delete_user_uc = Arc::new(DeleteUserUseCase::new(Arc::clone(&user_service)));

    // Warfare use cases
    let spawn_unit_uc = Arc::new(SpawnUnitUseCase::new(Arc::clone(&unit_service)));
    let list_units_uc = Arc::new(ListUnitsUseCase::new(Arc::clone(&unit_service)));

    // Economy use cases
    let get_corporation_uc = Arc::new(GetCorporationUseCase::new(Arc::clone(&corporation_service)));

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
