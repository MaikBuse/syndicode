use std::sync::Arc;

use crate::{domain::model::interface::UserRole, service::interface::InterfaceService};
use syndicode_proto::syndicode_interface_v1::{
    auth_service_server::AuthService, LoginRequest, LoginResponse, RegisterRequest,
    RegisterResponse,
};
use tonic::{async_trait, Request, Response, Status};

use super::common::service_error_into_status;

pub struct AuthPresenter {
    pub interface_service: Arc<InterfaceService>,
}

#[async_trait]
impl AuthService for AuthPresenter {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let request = request.into_inner();

        match self
            .interface_service
            .create_user(
                None,
                request.user_name,
                request.user_password,
                UserRole::Player,
                request.corporation_name,
            )
            .await
        {
            Ok(user) => Ok(Response::new(RegisterResponse {
                user_uuid: user.uuid.to_string(),
            })),
            Err(err) => Err(service_error_into_status(err)),
        }
    }
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let request = request.into_inner();

        let jwt = match self
            .interface_service
            .login(request.user_name, request.user_password)
            .await
        {
            Ok(user) => user,
            Err(err) => {
                return Err(service_error_into_status(err));
            }
        };

        Ok(Response::new(LoginResponse { jwt }))
    }
}
