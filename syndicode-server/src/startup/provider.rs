use crate::{
    application::{
        admin::{
            bootstrap::BootstrapAdminUseCase, create_user::CreateUserUseCase,
            delete_user::DeleteUserUseCase, get_user::GetUserUseCase,
        },
        auth::{
            login::LoginUseCase, register_user::RegisterUserUseCase,
            resend_verification::ResendVerificationUseCase, verify_user::VerifyUserUseCase,
        },
        economy::{
            acquire_listed_business::AcquireListedBusinessUseCase,
            bootstrap::BootstrapEconomyUseCase, get_corporation::GetCorporationUseCase,
            list_building_ownerships::ListBuildingOwnershipsUseCase,
            list_business_listings::ListBusinessListingUseCase,
            list_business_offers::ListBusinessOffersUseCase,
            list_businesses::ListBusinessesUseCase, list_corporations::ListCorporationsUseCase,
            list_markets::ListMarketsUseCase, query_buildings::QueryBuildingsUseCase,
            query_business_listings::QueryBusinessListingsUseCase,
        },
        game::get_game_tick::GetGameTickUseCase,
        init::InitializationOrchestrator,
        ports::{
            crypto::{JwtHandler, PasswordHandler},
            downloader::BackupDownloader,
            game_tick::GameTickRepository,
            init::InitializationRepository,
            leader::LeaderElector,
            limiter::RateLimitEnforcer,
            migration::MigrationRunner,
            outcome::OutcomeStoreReader,
            processor::GameTickProcessable,
            queuer::ActionQueueable,
            restorer::DatabaseRestorer,
            uow::UnitOfWork,
            verification::VerificationSendable,
        },
        processor::GameTickProcessor,
        warfare::{
            list_units::ListUnitsUseCase, list_units_by_corporation::ListUnitsByCorporationUseCase,
            spawn_unit::SpawnUnitUseCase,
        },
    },
    config::ServerConfig,
    domain::{
        economy::{
            building::repository::BuildingRepository,
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
        http::HttpBackupDownloader,
        postgres::{
            economy::{
                building::PgBuildingService, building_ownership::PgBuildingOwnershipService,
                business::PgBusinessService, business_listing::PgBusinessListingService,
                business_offer::PgBusinessOfferService, corporation::PgCorporationService,
                market::PgMarketService,
            },
            game_tick::PgGameTickService,
            init::PgInitializationService,
            migration::PostgresMigrator,
            unit::PgUnitService,
            uow::PostgresUnitOfWork,
            user::{PgUserRepository, PgUserService},
            PostgresDatabase,
        },
        restorer::PgRestoreExecutor,
        valkey::ValkeyStore,
    },
    presentation::{
        admin::AdminPresenter,
        auth::AuthPresenter,
        economy::EconomyPresenter,
        game::{user_channel_guard::UserChannels, GamePresenter},
    },
};
use std::sync::Arc;
use tokio::sync::Mutex;

// Represents the standard configuration of the application state
pub type DefaultProvider = AppProvider<
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
        PgBusinessOfferService,
        PgBuildingOwnershipService,
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
    PostgresMigrator,
    PgBuildingService,
    HttpBackupDownloader,
    PgRestoreExecutor,
>;

pub struct AppProvider<
    INI,
    G,
    P,
    J,
    Q,
    R,
    L,
    UOW,
    USR,
    UNT,
    CRP,
    RSR,
    GTR,
    VS,
    BL,
    M,
    BUI,
    DOW,
    RES,
> where
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
    M: MigrationRunner + 'static,
    BUI: BuildingRepository + 'static,
    DOW: BackupDownloader + 'static,
    RES: DatabaseRestorer + 'static,
{
    pub game_tick_processor: Arc<G>,
    pub leader_elector: Arc<L>,
    pub crypto: Arc<CryptoService>,
    pub initialization_orchestrator: Arc<InitializationOrchestrator<UOW, INI, RES, DOW, P, M>>,
    pub game_presenter: GamePresenter<R, Q, UNT, CRP, RSR, GTR, BL>,
    pub admin_presenter: AdminPresenter<Q, R, P, USR, CRP>,
    pub auth_presenter: AuthPresenter<R, P, J, UOW, USR, VS, Q, CRP>,
    pub economy_presenter: EconomyPresenter<R, BUI, CRP>,
}

impl DefaultProvider {
    pub async fn build_services(
        config: Arc<ServerConfig>,
        pg_db: Arc<PostgresDatabase>,
        valkey: Arc<ValkeyStore>,
        user_channels: UserChannels,
    ) -> anyhow::Result<DefaultProvider> {
        tracing::info!("Setting up the provider...");

        // Crypto service
        let crypto = Arc::new(CryptoService::new(config.clone())?);

        // Unit of Work
        let uow = Arc::new(PostgresUnitOfWork::new(pg_db.clone()));

        // Email Handler
        let sendable = Arc::new(EmailHandler::new(config.clone())?);

        // HTTP Downloader
        let http_downloader = Arc::new(HttpBackupDownloader::new());

        // Restorer
        let pg_restorer = Arc::new(PgRestoreExecutor::new(config.clone()));

        // Database Services
        let init_service = Arc::new(PgInitializationService::new(pg_db.clone()));
        let game_tick_service = Arc::new(PgGameTickService::new(pg_db.clone()));
        let user_service = Arc::new(PgUserService::new(pg_db.clone(), PgUserRepository));
        let unit_service = Arc::new(PgUnitService::new(pg_db.clone()));
        let corporation_service = Arc::new(PgCorporationService::new(pg_db.clone()));
        let market_service = Arc::new(PgMarketService::new(pg_db.clone()));
        let business_service = Arc::new(PgBusinessService::new(pg_db.clone()));
        let business_listing_service = Arc::new(PgBusinessListingService::new(pg_db.clone()));
        let business_offer_service = Arc::new(PgBusinessOfferService::new(pg_db.clone()));
        let building_service = Arc::new(PgBuildingService::new(pg_db.clone()));
        let building_ownership_service = Arc::new(PgBuildingOwnershipService::new(pg_db.clone()));

        // System use cases
        let get_game_tick_uc = Arc::new(
            GetGameTickUseCase::<PgGameTickService>::builder()
                .game_tick_repo(game_tick_service.clone())
                .build(),
        );

        // Auth use cases
        let register_user_uc = Arc::new(
            RegisterUserUseCase::builder()
                .uow(uow.clone())
                .pw(crypto.clone())
                .verification(sendable.clone())
                .corp_repo(corporation_service.clone())
                .action_queuer(valkey.clone())
                .build(),
        );
        let login_uc = Arc::new(LoginUseCase::new(
            crypto.clone(),
            crypto.clone(),
            user_service.clone(),
        ));
        let get_user_uc = Arc::new(
            GetUserUseCase::builder()
                .user_repo(user_service.clone())
                .build(),
        );

        // Admin use cases
        let bootstrap_admin_uc = Arc::new(
            BootstrapAdminUseCase::builder()
                .uow(uow.clone())
                .pw(crypto.clone())
                .init_repo(init_service.clone())
                .build(),
        );
        let create_user_uc = Arc::new(
            CreateUserUseCase::builder()
                .pw(crypto.clone())
                .user_repo(user_service.clone())
                .action_queuer(valkey.clone())
                .corp_repo(corporation_service.clone())
                .build(),
        );
        let delete_user_uc = Arc::new(
            DeleteUserUseCase::builder()
                .user_repo(user_service.clone())
                .action_queuer(valkey.clone())
                .corporation_repo(corporation_service.clone())
                .build(),
        );

        // Warfare use cases
        let list_units_uc = Arc::new(
            ListUnitsUseCase::builder()
                .unit_repository(unit_service.clone())
                .build(),
        );
        let list_units_by_corporation_uc = Arc::new(
            ListUnitsByCorporationUseCase::builder()
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
                .config(config.clone())
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
        let list_business_offers_uc = Arc::new(
            ListBusinessOffersUseCase::builder()
                .business_offer_repo(business_offer_service.clone())
                .build(),
        );
        let list_building_ownerships = Arc::new(
            ListBuildingOwnershipsUseCase::builder()
                .building_ownership_repo(building_ownership_service.clone())
                .build(),
        );
        let query_business_listings_uc = Arc::new(
            QueryBusinessListingsUseCase::builder()
                .business_listing_repo(business_listing_service.clone())
                .build(),
        );
        let query_buildings_uc = Arc::new(
            QueryBuildingsUseCase::builder()
                .building_repo(building_service.clone())
                .build(),
        );

        // Bootstrap
        let migrator = Arc::new(PostgresMigrator::new(pg_db.clone()));
        let initialization_orchestrator = Arc::new(
            InitializationOrchestrator::builder()
                .config(config.clone())
                .migrator(migrator)
                .init_repo(init_service.clone())
                .restorer(pg_restorer)
                .downloader(http_downloader)
                .bootstrap_admin_uc(bootstrap_admin_uc)
                .bootstrap_economy_uc(bootstrap_economy_uc)
                .build(),
        );

        let simulation = Arc::new(SimulationService);
        let game_state = Arc::new(Mutex::new(None));
        let game_tick_processor = Arc::new(
            GameTickProcessor::builder()
                .action_puller(valkey.clone())
                .outcome_notifier(valkey.clone())
                .outcome_store_writer(valkey.clone())
                .simulation(simulation.clone())
                .game_tick_repo(game_tick_service.clone())
                .state(game_state)
                .list_corporations_uc(list_corporations_uc.clone())
                .list_units_uc(list_units_uc.clone())
                .uow(uow.clone())
                .init_repo(init_service.clone())
                .list_markets_uc(list_markets_uc.clone())
                .list_businesses_uc(list_businesses_uc.clone())
                .list_business_listings_uc(list_business_listings_uc.clone())
                .list_business_offers_uc(list_business_offers_uc.clone())
                .list_building_ownerships(list_building_ownerships)
                .build(),
        );

        // Presenter
        let game_presenter = GamePresenter::builder()
            .limit(valkey.clone())
            .user_channels(user_channels.clone())
            .get_game_tick_uc(get_game_tick_uc.clone())
            .list_units_by_corporation_uc(list_units_by_corporation_uc.clone())
            .outcome_store_reader(valkey.clone())
            .spawn_unit_uc(spawn_unit_uc.clone())
            .get_corporation_uc(get_corporation_uc.clone())
            .valkey_client(valkey.get_client())
            .acquire_listed_business_uc(acquire_listed_business_uc.clone())
            .query_business_listings_uc(query_business_listings_uc.clone())
            .build();

        let admin_presenter = AdminPresenter::builder()
            .limit(valkey.clone())
            .create_user_uc(create_user_uc.clone())
            .get_user_uc(get_user_uc.clone())
            .delete_user_uc(delete_user_uc.clone())
            .build();

        let auth_presenter = AuthPresenter::builder()
            .limit(valkey.clone())
            .register_user_uc(register_user_uc)
            .get_user_uc(get_user_uc)
            .login_uc(login_uc.clone())
            .verify_user_uc(verify_user_uc.clone())
            .resend_verification_uc(resend_verification_uc.clone())
            .build();

        let economy_presenter = EconomyPresenter::builder()
            .query_buildings_uc(query_buildings_uc.clone())
            .get_corporation_uc(get_corporation_uc.clone())
            .limit(valkey.clone())
            .build();

        Ok(AppProvider {
            leader_elector: valkey.clone(),
            crypto,
            initialization_orchestrator,
            game_presenter,
            admin_presenter,
            auth_presenter,
            economy_presenter,
            game_tick_processor,
        })
    }
}
