use crate::{
    application::{
        action::ActionHandler,
        admin::{
            bootstrap_admin::BootstrapAdminUseCase, create_user::CreateUserUseCase,
            delete_user::DeleteUserUseCase,
        },
        auth::login::LoginUseCase,
        economy::get_corporation::GetCorporationUseCase,
        ports::{
            crypto::{JwtHandler, PasswordHandler},
            leader::LeaderElector,
            limiter::RateLimitEnforcer,
            processor::GameTickProcessable,
            queue::ActionQueuer,
            uow::UnitOfWork,
        },
        processor::GameTickProcessor,
        warfare::{list_units::ListUnitsUseCase, spawn_unit::SpawnUnitUseCase},
    },
    config::Config,
    domain::{
        corporation::repository::CorporationRepository, unit::repository::UnitRepository,
        user::repository::UserRepository,
    },
    infrastructure::{
        crypto::CryptoService,
        postgres::{
            corporation::PgCorporationService,
            unit::PgUnitService,
            uow::PostgresUnitOfWork,
            user::{PgUserRepository, PgUserService},
        },
        valkey::ValkeyStore,
    },
    presentation::{admin::AdminPresenter, auth::AuthPresenter, game::GamePresenter},
};
use dashmap::DashMap;
use sqlx::PgPool;
use std::sync::Arc;
use syndicode_proto::syndicode_interface_v1::GameUpdate;
use tokio::sync::mpsc::Sender;
use tonic::Status;
use uuid::Uuid;

// Represents the standard configuration of the application state
pub type DefaultAppState = AppState<
    GameTickProcessor,
    CryptoService,
    CryptoService,
    ValkeyStore,
    ValkeyStore,
    ValkeyStore,
    PostgresUnitOfWork,
    PgUserService,
    PgUnitService,
    PgCorporationService,
>;

pub struct AppState<G, P, J, Q, R, L, UOW, USR, UNT, CRP>
where
    G: GameTickProcessable + 'static,
    P: PasswordHandler + 'static,
    J: JwtHandler + 'static,
    Q: ActionQueuer + 'static,
    R: RateLimitEnforcer + 'static,
    L: LeaderElector + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
    UNT: UnitRepository + 'static,
    CRP: CorporationRepository + 'static,
{
    pub game_tick_processor: Arc<G>,
    pub user_channels: Arc<DashMap<Uuid, Sender<Result<GameUpdate, Status>>>>,
    pub action_handler: Arc<ActionHandler<Q>>,
    pub leader_elector: Arc<L>,
    pub crypto: Arc<CryptoService>,
    pub user_service: Arc<USR>,
    pub unit_service: Arc<UNT>,
    pub corporation_service: Arc<CRP>,
    pub login_uc: Arc<LoginUseCase<P, J, USR>>,
    pub bootstrap_admin_uc: Arc<BootstrapAdminUseCase<UOW, P>>,
    pub create_user_uc: Arc<CreateUserUseCase<P, UOW, USR>>,
    pub delete_user_uc: Arc<DeleteUserUseCase<USR>>,
    pub spawn_unit_uc: Arc<SpawnUnitUseCase<UNT>>,
    pub list_units_uc: Arc<ListUnitsUseCase<UNT>>,
    pub get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    pub game_presenter: GamePresenter<R, Q, UNT, CRP>,
    pub admin_presenter: AdminPresenter<P, UOW, USR>,
    pub auth_presenter: AuthPresenter<P, J, UOW, USR>,
}

impl DefaultAppState {
    pub async fn build_services(
        config: Arc<Config>,
        pg_pool: Arc<PgPool>,
        valkey: Arc<ValkeyStore>,
    ) -> anyhow::Result<DefaultAppState> {
        let user_channels = Arc::new(DashMap::new());

        let leader_elector = valkey.clone();

        // Action handler
        let action_handler = Arc::new(ActionHandler::new(valkey.clone()));

        // Crypto service
        let crypto = Arc::new(CryptoService::new()?);

        // Unit of Work
        let uow = Arc::new(PostgresUnitOfWork::new(Arc::clone(&pg_pool)));

        // Database Services
        let user_service = Arc::new(PgUserService::new(Arc::clone(&pg_pool), PgUserRepository));
        let unit_service = Arc::new(PgUnitService::new(Arc::clone(&pg_pool)));
        let corporation_service = Arc::new(PgCorporationService::new(Arc::clone(&pg_pool)));

        // Auth use cases
        let login_uc = Arc::new(LoginUseCase::new(
            crypto.clone(),
            crypto.clone(),
            user_service.clone(),
        ));

        // Admin use cases
        let bootstrap_admin_uc =
            Arc::new(BootstrapAdminUseCase::new(crypto.clone(), Arc::clone(&uow)));
        let create_user_uc = Arc::new(CreateUserUseCase::new(
            crypto.clone(),
            Arc::clone(&uow),
            Arc::clone(&user_service),
        ));
        let delete_user_uc = Arc::new(DeleteUserUseCase::new(Arc::clone(&user_service)));

        // Warfare use cases
        let spawn_unit_uc = Arc::new(SpawnUnitUseCase::new(Arc::clone(&unit_service)));
        let list_units_uc = Arc::new(ListUnitsUseCase::new(Arc::clone(&unit_service)));

        // Economy use cases
        let get_corporation_uc =
            Arc::new(GetCorporationUseCase::new(Arc::clone(&corporation_service)));

        let game_tick_processor = Arc::new(GameTickProcessor::new());

        // Presenter
        let game_presenter = GamePresenter {
            config: Arc::clone(&config),
            limit: valkey.clone(),
            action_handler: Arc::clone(&action_handler),
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

        Ok(AppState {
            action_handler,
            leader_elector,
            user_channels,
            crypto,
            user_service,
            unit_service,
            corporation_service,
            login_uc,
            bootstrap_admin_uc,
            create_user_uc,
            delete_user_uc,
            spawn_unit_uc,
            list_units_uc,
            get_corporation_uc,
            game_presenter,
            admin_presenter,
            auth_presenter,
            game_tick_processor,
        })
    }
}
