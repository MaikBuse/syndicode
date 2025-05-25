use crate::{
    application::{
        admin::{
            bootstrap_admin::BootstrapAdminUseCase, create_user::CreateUserUseCase,
            delete_user::DeleteUserUseCase,
        },
        auth::{
            login::LoginUseCase, resend_verification::ResendVerificationUseCase,
            verify_user::VerifyUserUseCase,
        },
        economy::{
            acquire_listed_business::AcquireListedBusinessUseCase,
            bootstrap_economy::BootstrapEconomyUseCase, get_corporation::GetCorporationUseCase,
            list_business_listings::ListBusinessListingUseCase,
            list_businesses::ListBusinessesUseCase, list_corporations::ListCorporationsUseCase,
            list_markets::ListMarketsUseCase,
            query_business_listings::QueryBusinessListingsUseCase,
        },
        ports::{
            crypto::{JwtHandler, PasswordHandler},
            game_tick::GameTickRepository,
            init::InitializationRepository,
            leader::LeaderElector,
            limiter::RateLimitEnforcer,
            outcome::OutcomeStoreReader,
            processor::GameTickProcessable,
            queuer::ActionQueueable,
            uow::UnitOfWork,
            verification::VerificationSendable,
        },
        processor::GameTickProcessor,
        warfare::{
            list_units::ListUnitsUseCase, list_units_by_user::ListUnitsByUserUseCase,
            spawn_unit::SpawnUnitUseCase,
        },
    },
    config::Config,
    domain::{
        economy::{
            business_listing::repository::BusinessListingRepository,
            corporation::repository::CorporationRepository,
        },
        simulation::SimulationService,
        unit::repository::UnitRepository,
        user::repository::UserRepository,
    },
    infrastructure::{
        crypto::CryptoService,
        email::EmailHandler,
        postgres::{
            economy::{
                business::PgBusinessService, business_listing::PgBusinessListingService,
                corporation::PgCorporationService, market::PgMarketService,
            },
            game_tick::PgGameTickService,
            init::PgInitializationService,
            unit::PgUnitService,
            uow::PostgresUnitOfWork,
            user::{PgUserRepository, PgUserService},
        },
        valkey::ValkeyStore,
    },
    presentation::{
        admin::AdminPresenter,
        auth::AuthPresenter,
        game::{user_channel_guard::UserChannels, GamePresenter},
    },
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::OnceCell;

// Represents the standard configuration of the application state
pub type DefaultAppState = AppState<
    PgInitializationService,
    GameTickProcessor<
        PgInitializationService,
        SimulationService,
        ValkeyStore,
        ValkeyStore,
        ValkeyStore,
        PostgresUnitOfWork,
        PgGameTickService,
        PgUnitService,
        PgCorporationService,
        PgMarketService,
        PgBusinessService,
        PgBusinessListingService,
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
    EmailHandler,
    PgBusinessListingService,
>;

pub struct AppState<INI, G, P, J, Q, R, L, UOW, USR, UNT, CRP, RSR, GTR, VS, BL>
where
    INI: InitializationRepository + 'static,
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
    RSR: OutcomeStoreReader + 'static,
    GTR: GameTickRepository + 'static,
    VS: VerificationSendable + 'static,
    BL: BusinessListingRepository + 'static,
{
    pub game_tick_processor: Arc<G>,
    pub leader_elector: Arc<L>,
    pub crypto: Arc<CryptoService>,
    pub init_service: Arc<INI>,
    pub user_service: Arc<USR>,
    pub unit_service: Arc<UNT>,
    pub corporation_service: Arc<CRP>,
    pub login_uc: Arc<LoginUseCase<P, J, USR>>,
    pub bootstrap_admin_uc: Arc<BootstrapAdminUseCase<UOW, P>>,
    pub bootstrap_economy_uc: Arc<BootstrapEconomyUseCase<UOW, INI>>,
    pub create_user_uc: Arc<CreateUserUseCase<P, UOW, USR, VS>>,
    pub delete_user_uc: Arc<DeleteUserUseCase<USR>>,
    pub list_units_uc: Arc<ListUnitsUseCase<UNT>>,
    pub get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    pub game_presenter: GamePresenter<R, Q, UNT, CRP, RSR, GTR, BL>,
    pub admin_presenter: AdminPresenter<R, P, UOW, USR, VS>,
    pub auth_presenter: AuthPresenter<R, P, J, UOW, USR, VS>,
}

impl DefaultAppState {
    pub async fn build_services(
        config: Arc<Config>,
        pg_pool: Arc<PgPool>,
        valkey: Arc<ValkeyStore>,
        user_channels: UserChannels,
    ) -> anyhow::Result<DefaultAppState> {
        // Crypto service
        let crypto = Arc::new(CryptoService::new()?);

        // Unit of Work
        let uow = Arc::new(PostgresUnitOfWork::new(Arc::clone(&pg_pool)));

        // Email Handler
        let sendable = Arc::new(EmailHandler::new());

        // Database Services
        let init_service = Arc::new(PgInitializationService::new(pg_pool.clone()));
        let game_tick_service = Arc::new(PgGameTickService::new(pg_pool.clone()));
        let user_service = Arc::new(PgUserService::new(Arc::clone(&pg_pool), PgUserRepository));
        let unit_service = Arc::new(PgUnitService::new(Arc::clone(&pg_pool)));
        let corporation_service = Arc::new(PgCorporationService::new(Arc::clone(&pg_pool)));
        let market_service = Arc::new(PgMarketService::new(pg_pool.clone()));
        let business_service = Arc::new(PgBusinessService::new(pg_pool.clone()));
        let business_listing_service = Arc::new(PgBusinessListingService::new(pg_pool.clone()));

        // Auth use cases
        let login_uc = Arc::new(LoginUseCase::new(
            crypto.clone(),
            crypto.clone(),
            user_service.clone(),
        ));

        // Admin use cases
        let bootstrap_admin_uc = Arc::new(
            BootstrapAdminUseCase::builder()
                .uow(uow.clone())
                .pw(crypto.clone())
                .build(),
        );
        let create_user_uc = Arc::new(
            CreateUserUseCase::builder()
                .uow(uow.clone())
                .pw(crypto.clone())
                .verification(sendable.clone())
                .user_repo(user_service.clone())
                .build(),
        );
        let delete_user_uc = Arc::new(
            DeleteUserUseCase::builder()
                .user_repo(user_service.clone())
                .build(),
        );

        // Warfare use cases
        let list_units_uc = Arc::new(
            ListUnitsUseCase::builder()
                .unit_repository(unit_service.clone())
                .build(),
        );
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
        let verify_user_uc = Arc::new(VerifyUserUseCase::builder().uow(uow.clone()).build());
        let resend_verification_uc = Arc::new(
            ResendVerificationUseCase::builder()
                .uow(uow.clone())
                .verification(sendable.clone())
                .build(),
        );

        // Economy use cases
        let acquire_listed_business_uc = Arc::new(
            AcquireListedBusinessUseCase::builder()
                .action_queuer(valkey.clone())
                .game_tick_repo(game_tick_service.clone())
                .build(),
        );
        let bootstrap_economy_uc = Arc::new(
            BootstrapEconomyUseCase::builder()
                .uow(uow.clone())
                .init_repo(init_service.clone())
                .build(),
        );
        let get_corporation_uc = Arc::new(
            GetCorporationUseCase::builder()
                .corporation_repo(corporation_service.clone())
                .build(),
        );
        let list_corporations_uc = Arc::new(
            ListCorporationsUseCase::builder()
                .corporation_repo(corporation_service.clone())
                .build(),
        );
        let list_markets_uc = Arc::new(
            ListMarketsUseCase::builder()
                .market_repo(market_service.clone())
                .build(),
        );
        let list_businesses_uc = Arc::new(
            ListBusinessesUseCase::builder()
                .business_repo(business_service.clone())
                .build(),
        );
        let list_business_listings_uc = Arc::new(
            ListBusinessListingUseCase::builder()
                .business_listing_repo(business_listing_service.clone())
                .build(),
        );
        let query_business_listings_uc = Arc::new(
            QueryBusinessListingsUseCase::builder()
                .business_listing_repo(business_listing_service.clone())
                .build(),
        );

        let simulation = Arc::new(SimulationService);
        let game_tick_processor = Arc::new(
            GameTickProcessor::builder()
                .action_puller(valkey.clone())
                .outcome_notifier(valkey.clone())
                .outcome_store_writer(valkey.clone())
                .simulation(simulation.clone())
                .game_tick_repo(game_tick_service.clone())
                .list_corporations_uc(list_corporations_uc.clone())
                .list_units_uc(list_units_uc.clone())
                .uow(uow.clone())
                .init_repo(init_service.clone())
                .init_check_cell(OnceCell::new())
                .list_markets_uc(list_markets_uc.clone())
                .list_businesses_uc(list_businesses_uc.clone())
                .list_business_listings_uc(list_business_listings_uc.clone())
                .build(),
        );

        // Presenter
        let game_presenter = GamePresenter::builder()
            .config(config.clone())
            .limit(valkey.clone())
            .user_channels(user_channels.clone())
            .list_units_by_user_uc(list_units_by_user_uc.clone())
            .outcome_store_reader(valkey.clone())
            .spawn_unit_uc(spawn_unit_uc.clone())
            .get_corporation_uc(get_corporation_uc.clone())
            .valkey_client(valkey.get_client())
            .acquire_listed_business_uc(acquire_listed_business_uc.clone())
            .query_business_listings_uc(query_business_listings_uc.clone())
            .build();

        let admin_presenter = AdminPresenter::builder()
            .config(config.clone())
            .limit(valkey.clone())
            .create_user_uc(create_user_uc.clone())
            .delete_user_uc(delete_user_uc.clone())
            .build();

        let auth_presenter = AuthPresenter::builder()
            .config(config.clone())
            .limit(valkey.clone())
            .create_user_uc(create_user_uc.clone())
            .login_uc(login_uc.clone())
            .verify_user_uc(verify_user_uc.clone())
            .resend_verification_uc(resend_verification_uc.clone())
            .build();

        Ok(AppState {
            init_service,
            leader_elector: valkey.clone(),
            crypto,
            user_service,
            unit_service,
            corporation_service,
            login_uc,
            bootstrap_admin_uc,
            bootstrap_economy_uc,
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
