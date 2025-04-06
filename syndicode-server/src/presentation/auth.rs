use super::common::application_error_into_status;
use crate::{
    application::{
        admin::create_user::CreateUserUseCase,
        auth::login::LoginUseCase,
        ports::{
            crypto::{JwtHandler, PasswordHandler},
            uow::UnitOfWork,
        },
    },
    domain::user::{model::role::UserRole, repository::UserRepository},
};
use std::sync::Arc;
use syndicode_proto::syndicode_interface_v1::{
    auth_service_server::AuthService, LoginRequest, LoginResponse, RegisterRequest,
    RegisterResponse,
};
use tonic::{async_trait, Request, Response, Status};

pub struct AuthPresenter<P, J, UOW, USR>
where
    P: PasswordHandler + 'static,
    J: JwtHandler + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
{
    pub create_user_uc: Arc<CreateUserUseCase<P, UOW, USR>>,
    pub login_uc: Arc<LoginUseCase<P, J, USR>>,
}

#[async_trait]
impl<P, J, UOW, USR> AuthService for AuthPresenter<P, J, UOW, USR>
where
    P: PasswordHandler + 'static,
    J: JwtHandler + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
{
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
