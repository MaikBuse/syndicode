mod application;
mod domain;
mod engine;
mod infrastructure;
mod presentation;
mod startup;

use application::admin::create_user::CreateUserUseCase;
use application::admin::delete_user::DeleteUserUseCase;
use application::auth::login::LoginUseCase;
use application::economy::get_corporation::GetCorporationUseCase;
use application::warfare::list_units::ListUnitsUseCase;
use application::warfare::spawn_unit::SpawnUnitUseCase;
use dashmap::DashMap;
use domain::repository::corporation::CorporationRepository;
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

const JOB_INTERVAL: Duration = Duration::from_secs(3);

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // Add health checks for servers
    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<AdminServiceServer<AdminPresenter<PostgresUnitOfWork>>>()
        .await;

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

    Ok(())
}
