use crate::{
    application::{
        admin::{create_user::CreateUserUseCase, delete_user::DeleteUserUseCase},
        auth::login::LoginUseCase,
        economy::get_corporation::GetCorporationUseCase,
        warfare::{list_units::ListUnitsUseCase, spawn_unit::SpawnUnitUseCase},
    },
    domain::{
        corporation::repository::CorporationRepository, unit::repository::UnitRepository,
        user::repository::UserRepository,
    },
    engine::{Engine, Job},
    infrastructure::{
        crypto::CryptoService,
        postgres::{
            corporation::PgCorporationService,
            unit::PgUnitService,
            uow::PostgresUnitOfWork,
            user::{PgUserRepository, PgUserService},
        },
    },
};
use dashmap::DashMap;
use sqlx::PgPool;
use std::{collections::VecDeque, sync::Arc};
use syndicode_proto::syndicode_interface_v1::GameUpdate;
use tokio::sync::{mpsc::Sender, Mutex};
use tonic::Status;
use uuid::Uuid;

pub struct AppState {
    pub engine: Arc<Mutex<Engine>>,
    pub jobs: Arc<Mutex<VecDeque<Job>>>,
    pub user_channels: Arc<DashMap<Uuid, Sender<Result<GameUpdate, Status>>>>,
    pub crypto: Arc<CryptoService>,
    pub user_service: Arc<dyn UserRepository>,
    pub unit_service: Arc<dyn UnitRepository>,
    pub corporation_service: Arc<dyn CorporationRepository>,
    pub login_uc: Arc<LoginUseCase>,
    pub create_user_uc: Arc<CreateUserUseCase<PostgresUnitOfWork>>,
    pub delete_user_uc: Arc<DeleteUserUseCase>,
    pub spawn_unit_uc: Arc<SpawnUnitUseCase>,
    pub list_units_uc: Arc<ListUnitsUseCase>,
    pub get_corporation_uc: Arc<GetCorporationUseCase>,
}

pub async fn build_services(pool: Arc<PgPool>) -> AppState {
    let jobs = Arc::new(Mutex::new(VecDeque::new()));
    let user_channels = Arc::new(DashMap::new());

    // Crypto service
    let crypto = Arc::new(CryptoService::new());

    // Unit of Work
    let uow = Arc::new(PostgresUnitOfWork::new(Arc::clone(&pool)));

    // Database Services
    let user_service: Arc<dyn UserRepository> =
        Arc::new(PgUserService::new(Arc::clone(&pool), PgUserRepository));
    let unit_service: Arc<dyn UnitRepository> = Arc::new(PgUnitService::new(Arc::clone(&pool)));
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
    let engine = Arc::new(Mutex::new(Engine::init(
        Arc::clone(&jobs),
        Arc::clone(&spawn_unit_uc),
    )));

    AppState {
        engine,
        jobs,
        user_channels,
        crypto,
        user_service,
        unit_service,
        corporation_service,
        login_uc,
        create_user_uc,
        delete_user_uc,
        spawn_unit_uc,
        list_units_uc,
        get_corporation_uc,
    }
}
