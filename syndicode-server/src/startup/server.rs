use super::services::AppState;
use crate::{
    config::Config,
    infrastructure::{postgres::uow::PostgresUnitOfWork, valkey::ValkeyStore},
    presentation::{
        admin::AdminPresenter, auth::AuthPresenter, game::GamePresenter,
        middleware::MiddlewareLayer,
    },
};
use std::sync::Arc;
use syndicode_proto::syndicode_interface_v1::{
    admin_service_server::AdminServiceServer, auth_service_server::AuthServiceServer,
    game_service_server::GameServiceServer,
};
use tonic::transport::Server;

const SOCKET_ADDR: &str = "[::]:50051";

pub async fn start_grpc_services(
    config: Arc<Config>,
    app: AppState,
    valkey: Arc<ValkeyStore>,
) -> anyhow::Result<()> {
    let addr = SOCKET_ADDR.parse()?;

    // Add health checks for servers
    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<AdminServiceServer<AdminPresenter<PostgresUnitOfWork>>>()
        .await;

    // Setup reflection service for service discovery
    let reflection_service = syndicode_proto::create_reflection_service()?;

    let game_presenter = GamePresenter {
        config: Arc::clone(&config),
        limit: valkey.clone(),
        action_handler: Arc::clone(&app.action_handler),
        user_channels: Arc::clone(&app.user_channels),
        list_units_uc: Arc::clone(&app.list_units_uc),
        get_corporation_uc: Arc::clone(&app.get_corporation_uc),
    };

    let admin_presenter = AdminPresenter {
        create_user_uc: Arc::clone(&app.create_user_uc),
        delete_user_uc: Arc::clone(&app.delete_user_uc),
    };

    let auth_presenter = AuthPresenter {
        create_user_uc: Arc::clone(&app.create_user_uc),
        login_uc: Arc::clone(&app.login_uc),
    };

    tracing::info!("Starting server...");

    Server::builder()
        .layer(MiddlewareLayer::new(
            Arc::clone(&config),
            Arc::clone(&app.crypto),
            Arc::clone(&valkey),
        ))
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(GameServiceServer::new(game_presenter))
        .add_service(AdminServiceServer::new(admin_presenter))
        .add_service(AuthServiceServer::new(auth_presenter))
        .serve(addr)
        .await?;

    Ok(())
}
