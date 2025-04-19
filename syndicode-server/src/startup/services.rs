use crate::{
    application::{
        admin::{
            bootstrap_admin::BootstrapAdminUseCase, create_user::CreateUserUseCase,
            delete_user::DeleteUserUseCase,
        },
        auth::login::LoginUseCase,
        economy::{
            get_corporation::GetCorporationUseCase, list_corporations::ListCorporationsUseCase,
        },
        ports::{
            crypto::{JwtHandler, PasswordHandler},
            game_tick::GameTickRepository,
            leader::LeaderElector,
            limiter::RateLimitEnforcer,
            processor::GameTickProcessable,
            queuer::ActionQueueable,
            results::ResultStoreReader,
            uow::UnitOfWork,
        },
        processor::GameTickProcessor,
        warfare::{
            list_units::ListUnitsUseCase, list_units_by_user::ListUnitsByUserUseCase,
            spawn_unit::SpawnUnitUseCase,
        },
    },
    config::Config,
    domain::{
        corporation::repository::CorporationRepository, simulation::SimulationService,
        unit::repository::UnitRepository, user::repository::UserRepository,
    },
    infrastructure::{
        crypto::CryptoService,
        postgres::{
            corporation::PgCorporationService,
            game_tick::PgGameTickService,
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
    GameTickProcessor<
        SimulationService,
        ValkeyStore,
        ValkeyStore,
        ValkeyStore,
        PostgresUnitOfWork,
        PgGameTickService,
        PgUnitService,
        PgCorporationService,
    >,
    CryptoService,
    CryptoService,
    ValkeyStore,
    ValkeyStore,
    ValkeyStore,
    PostgresUnitOfWork,
    PgUserService,
    PgUnitService,
    PgCorporationService,
    ValkeyStore,
    PgGameTickService,
>;

pub struct AppState<G, P, J, Q, R, L, UOW, USR, UNT, CRP, RSR, GTR>
where
    G: GameTickProcessable + 'static,
    P: PasswordHandler + 'static,
    J: JwtHandler + 'static,
    Q: ActionQueueable + 'static,
    R: RateLimitEnforcer + 'static,
    L: LeaderElector + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
    UNT: UnitRepository + 'static,
    CRP: CorporationRepository + 'static,
    RSR: ResultStoreReader + 'static,
    GTR: GameTickRepository + 'static,
{
    pub game_tick_processor: Arc<G>,
    pub user_channels: Arc<DashMap<Uuid, Sender<Result<GameUpdate, Status>>>>,
    pub leader_elector: Arc<L>,
    pub crypto: Arc<CryptoService>,
    pub user_service: Arc<USR>,
    pub unit_service: Arc<UNT>,
    pub corporation_service: Arc<CRP>,
    pub login_uc: Arc<LoginUseCase<P, J, USR>>,
    pub bootstrap_admin_uc: Arc<BootstrapAdminUseCase<UOW, P>>,
    pub create_user_uc: Arc<CreateUserUseCase<P, UOW, USR>>,
    pub delete_user_uc: Arc<DeleteUserUseCase<USR>>,
    pub list_units_uc: Arc<ListUnitsUseCase<UNT>>,
    pub get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    pub game_presenter: GamePresenter<R, Q, UNT, CRP, RSR, GTR>,
    pub admin_presenter: AdminPresenter<R, P, UOW, USR>,
    pub auth_presenter: AuthPresenter<R, P, J, UOW, USR>,
}

impl DefaultAppState {
    pub async fn build_services(
        config: Arc<Config>,
        pg_pool: Arc<PgPool>,
        valkey: Arc<ValkeyStore>,
    ) -> anyhow::Result<DefaultAppState> {
        let user_channels = Arc::new(DashMap::new());

        // Crypto service
        let crypto = Arc::new(CryptoService::new()?);

        // Unit of Work
        let uow = Arc::new(PostgresUnitOfWork::new(Arc::clone(&pg_pool)));

        // Database Services
        let game_tick_service = Arc::new(PgGameTickService::new(pg_pool.clone()));
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
        let list_units_uc = Arc::new(ListUnitsUseCase::new(Arc::clone(&unit_service)));
        let list_units_by_user_uc = Arc::new(
            ListUnitsByUserUseCase::builder()
                .unit_repository(unit_service.clone())
                .build(),
        );
        let spawn_unit_uc = Arc::new(
            SpawnUnitUseCase::builder()
                .action_queuer(valkey.clone())
                .game_tick_repo(game_tick_service.clone())
                .build(),
        );

        // Economy use cases
        let get_corporation_uc =
            Arc::new(GetCorporationUseCase::new(Arc::clone(&corporation_service)));
        let list_corporations_uc =
            Arc::new(ListCorporationsUseCase::new(corporation_service.clone()));

        let simulation = Arc::new(SimulationService);
        let game_tick_processor = Arc::new(
            GameTickProcessor::builder()
                .action_puller(valkey.clone())
                .result_notifier(valkey.clone())
                .result_store_writer(valkey.clone())
                .simulation(simulation.clone())
                .game_tick_repo(game_tick_service.clone())
                .list_corporations_uc(list_corporations_uc.clone())
                .list_units_uc(list_units_uc.clone())
                .uow(uow.clone())
                .build(),
        );

        // Presenter
        let game_presenter = GamePresenter::builder()
            .config(config.clone())
            .limit(valkey.clone())
            .user_channels(user_channels.clone())
            .list_units_by_user_uc(list_units_by_user_uc.clone())
            .result_store_reader(valkey.clone())
            .spawn_unit_uc(spawn_unit_uc.clone())
            .get_corporation_uc(get_corporation_uc.clone())
            .valkey_client(valkey.get_client())
            .build();

        let admin_presenter = AdminPresenter {
            config: config.clone(),
            limit: valkey.clone(),
            create_user_uc: Arc::clone(&create_user_uc),
            delete_user_uc: Arc::clone(&delete_user_uc),
        };

        let auth_presenter = AuthPresenter {
            config: config.clone(),
            limit: valkey.clone(),
            create_user_uc: Arc::clone(&create_user_uc),
            login_uc: Arc::clone(&login_uc),
        };

        Ok(AppState {
            leader_elector: valkey.clone(),
            user_channels,
            crypto,
            user_service,
            unit_service,
            corporation_service,
            login_uc,
            bootstrap_admin_uc,
            create_user_uc,
            delete_user_uc,
            list_units_uc,
            get_corporation_uc,
            game_presenter,
            admin_presenter,
            auth_presenter,
            game_tick_processor,
        })
    }
}
