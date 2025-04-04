use crate::{
    application::{
        admin::{create_user::CreateUserUseCase, delete_user::DeleteUserUseCase},
        auth::login::LoginUseCase,
        economy::get_corporation::GetCorporationUseCase,
        warfare::{list_units::ListUnitsUseCase, spawn_unit::SpawnUnitUseCase},
    },
    domain::repository::{
        corporation::CorporationRepository, unit::UnitRepository, user::UserRepository,
    },
    infrastructure::{
        crypto::CryptoService,
        postgres::{
            corporation::PgCorporationService,
            unit::PgUnitService,
            uow::PostgresUnitOfWork,
            user::{PgUserRepository, PgUserService},
            PostgresDatabase,
        },
    },
};
use std::sync::Arc;

pub struct AppState {
    pub crypto: Arc<CryptoService>,
    pub user_service: Arc<dyn UserRepository>,
    pub unit_service: Arc<dyn UnitRepository>,
    pub corporation_service: Arc<dyn CorporationRepository>,
    pub login_uc: Arc<LoginUseCase>,
    pub create_user_uc: Arc<CreateUserUseCase>,
    pub delete_user_uc: Arc<DeleteUserUseCase>,
    pub spawn_unit_uc: Arc<SpawnUnitUseCase>,
    pub list_units_uc: Arc<ListUnitsUseCase>,
    pub get_corporation_uc: Arc<GetCorporationUseCase>,
}

pub async fn build_services(pool: Arc<PostgresDatabase>) -> AppState {
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

    AppState {
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
