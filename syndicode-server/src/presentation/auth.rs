use super::common::{application_error_into_status, check_rate_limit};
use crate::{
    application::{
        admin::create_user::CreateUserUseCase,
        auth::{
            login::LoginUseCase, resend_verification::ResendVerificationUseCase,
            verify_user::VerifyUserUseCase,
        },
        ports::{
            crypto::{JwtHandler, PasswordHandler},
            limiter::{LimiterCategory, RateLimitEnforcer},
            uow::UnitOfWork,
            verification::VerificationSendable,
        },
    },
    config::Config,
    domain::user::{model::role::UserRole, repository::UserRepository},
};
use bon::Builder;
use std::sync::Arc;
use syndicode_proto::syndicode_interface_v1::{
    auth_service_server::AuthService, LoginRequest, LoginResponse, RegisterRequest,
    RegisterResponse, ResendVerificationEmailRequest, ResendVerificationEmailResponse,
    VerifyUserRequest, VerifyUserResponse,
};
use tonic::{Request, Response, Status};

#[derive(Builder)]
pub struct AuthPresenter<R, P, J, UOW, USR, VS>
where
    R: RateLimitEnforcer + 'static,
    P: PasswordHandler + 'static,
    J: JwtHandler + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
    VS: VerificationSendable + 'static,
{
    config: Arc<Config>,
    limit: Arc<R>,
    create_user_uc: Arc<CreateUserUseCase<P, UOW, USR, VS>>,
    login_uc: Arc<LoginUseCase<P, J, USR>>,
    verify_user_uc: Arc<VerifyUserUseCase<UOW>>,
    resend_verification_uc: Arc<ResendVerificationUseCase<UOW, VS>>,
}

#[tonic::async_trait]
impl<R, P, J, UOW, USR, VS> AuthService for AuthPresenter<R, P, J, UOW, USR, VS>
where
    R: RateLimitEnforcer + 'static,
    P: PasswordHandler + 'static,
    J: JwtHandler + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
    VS: VerificationSendable + 'static,
{
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            &self.config.ip_address_header,
            LimiterCategory::Auth,
        )
        .await?;

        let request = request.into_inner();

        match self
            .create_user_uc
            .execute()
            .user_name(request.user_name)
            .user_email(request.email)
            .password(request.user_password)
            .user_role(UserRole::Player)
            .corporation_name(request.corporation_name)
            .call()
            .await
        {
            Ok(user) => Ok(Response::new(RegisterResponse {
                user_uuid: user.uuid.to_string(),
            })),
            Err(err) => Err(application_error_into_status(err)),
        }
    }

    async fn verify_user(
        &self,
        request: Request<VerifyUserRequest>,
    ) -> Result<Response<VerifyUserResponse>, Status> {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            &self.config.ip_address_header,
            LimiterCategory::Auth,
        )
        .await?;

        let request = request.into_inner();

        let user_uuid = self
            .verify_user_uc
            .execute(request.user_name, request.code)
            .await
            .map_err(application_error_into_status)?;

        Ok(Response::new(VerifyUserResponse {
            user_uuid: user_uuid.to_string(),
        }))
    }

    async fn resend_verification_email(
        &self,
        request: Request<ResendVerificationEmailRequest>,
    ) -> Result<Response<ResendVerificationEmailResponse>, Status> {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            &self.config.ip_address_header,
            LimiterCategory::Auth,
        )
        .await?;

        let user = self
            .resend_verification_uc
            .execute(request.into_inner().user_name)
            .await
            .map_err(application_error_into_status)?;

        Ok(Response::new(ResendVerificationEmailResponse {
            user_name: user.name.into_inner(),
            email: user.email.into_inner(),
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            &self.config.ip_address_header,
            LimiterCategory::Auth,
        )
        .await?;

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
