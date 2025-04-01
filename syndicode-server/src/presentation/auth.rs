use crate::{
    application::{admin::create_user::CreateUserUseCase, auth::login::LoginUseCase},
    domain::user::role::UserRole,
};
use std::sync::Arc;
use syndicode_proto::syndicode_interface_v1::{
    auth_service_server::AuthService, LoginRequest, LoginResponse, RegisterRequest,
    RegisterResponse,
};
use tonic::{async_trait, Request, Response, Status};

use super::common::application_error_into_status;

pub struct AuthPresenter {
    pub create_user_uc: Arc<CreateUserUseCase>,
    pub login_uc: Arc<LoginUseCase>,
}

#[async_trait]
impl AuthService for AuthPresenter {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let request = request.into_inner();

        match self
            .create_user_uc
            .execute(
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
            Err(err) => Err(application_error_into_status(err)),
        }
    }
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let request = request.into_inner();

        let jwt = match self
            .login_uc
            .execute(request.user_name, request.user_password)
            .await
        {
            Ok(user) => user,
            Err(err) => {
                return Err(application_error_into_status(err));
            }
        };

        Ok(Response::new(LoginResponse { jwt }))
    }
}
