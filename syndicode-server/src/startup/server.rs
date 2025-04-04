use tonic::transport::Server;

use super::services::AppState;

pub const SOCKET_ADDR: &str = "[::]:50051";

pub async fn start_grpc_services(app: AppState) -> Result<(), tonic::transport::Error> {
    let addr = SOCKET_ADDR.parse()?;

    tracing::info!("Starting server...");

    Server::builder()
        .layer(MiddlewareLayer::new(app))
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(GameServiceServer::new(game_presenter))
        .add_service(AdminServiceServer::new(admin_presenter))
        .add_service(AuthServiceServer::new(auth_presenter))
        .serve(addr)
        .await?;
}
