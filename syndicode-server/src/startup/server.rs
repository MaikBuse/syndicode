use super::services::DefaultAppState;
use crate::{
    config::Config,
    infrastructure::{
        crypto::CryptoService,
        postgres::{uow::PostgresUnitOfWork, user::PgUserService},
        valkey::ValkeyStore,
    },
    presentation::{admin::AdminPresenter, middleware::MiddlewareLayer},
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
    app: DefaultAppState,
    valkey: Arc<ValkeyStore>,
) -> anyhow::Result<()> {
    let addr = SOCKET_ADDR.parse()?;

    // Add health checks for servers
    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<AdminServiceServer<AdminPresenter<CryptoService, PostgresUnitOfWork, PgUserService>>>()
        .await;

    // Setup reflection service for service discovery
    let reflection_service = syndicode_proto::create_reflection_service()?;

    tracing::info!("Starting server...");

    Server::builder()
        .layer(MiddlewareLayer::new(
            Arc::clone(&config),
            Arc::clone(&app.crypto),
            Arc::clone(&valkey),
        ))
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(GameServiceServer::new(app.game_presenter))
        .add_service(AdminServiceServer::new(app.admin_presenter))
        .add_service(AuthServiceServer::new(app.auth_presenter))
        .serve(addr)
        .await?;

    Ok(())
}
