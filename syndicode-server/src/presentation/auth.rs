use super::{
    common::{check_rate_limit, uuid_from_metadata},
    error::PresentationError,
};
use crate::{
    application::{
        admin::get_user::GetUserUseCase,
        auth::{
            login::LoginUseCase, register_user::RegisterUserUseCase,
            resend_verification::ResendVerificationUseCase, verify_user::VerifyUserUseCase,
        },
        ports::{
            crypto::{JwtHandler, PasswordHandler},
            limiter::{LimiterCategory, RateLimitEnforcer},
            queuer::ActionQueueable,
            uow::UnitOfWork,
            verification::VerificationSendable,
        },
    },
    config::ServerConfig,
    domain::{
        economy::corporation::repository::CorporationRepository, user::repository::UserRepository,
    },
};
use bon::Builder;
use std::sync::Arc;
use syndicode_proto::syndicode_interface_v1::{
    auth_service_server::AuthService, GetCurrentUserRequest, GetUserResponse, LoginRequest,
    LoginResponse, RegisterRequest, RegisterResponse, ResendVerificationEmailRequest,
    ResendVerificationEmailResponse, VerifyUserRequest, VerifyUserResponse,
};
use tonic::{Request, Response, Status};

#[derive(Builder)]
pub struct AuthPresenter<R, P, J, UOW, USR, VS, Q, CRP>
where
    R: RateLimitEnforcer + 'static,
    P: PasswordHandler + 'static,
    J: JwtHandler + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
    VS: VerificationSendable + 'static,
    Q: ActionQueueable + 'static,
    CRP: CorporationRepository + 'static,
{
    config: Arc<ServerConfig>,
    limit: Arc<R>,
    get_user_uc: Arc<GetUserUseCase<USR>>,
    register_user_uc: Arc<RegisterUserUseCase<Q, UOW, P, VS, CRP>>,
    login_uc: Arc<LoginUseCase<P, J, USR>>,
    verify_user_uc: Arc<VerifyUserUseCase<UOW>>,
    resend_verification_uc: Arc<ResendVerificationUseCase<UOW, VS>>,
}

#[tonic::async_trait]
impl<R, P, J, UOW, USR, VS, Q, CRP> AuthService for AuthPresenter<R, P, J, UOW, USR, VS, Q, CRP>
where
    R: RateLimitEnforcer + 'static,
    P: PasswordHandler + 'static,
    J: JwtHandler + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
    VS: VerificationSendable + 'static,
    Q: ActionQueueable + 'static,
    CRP: CorporationRepository + 'static,
{
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            self.config.rate_limiter.ip_address_header.as_str(),
            LimiterCategory::Auth,
        )
        .await
        .map_err(|status| *status)?;

        let request = request.into_inner();

        match self
            .register_user_uc
            .execute()
            .user_name(request.user_name)
            .user_email(request.email)
            .password(request.user_password)
            .corporation_name(request.corporation_name)
            .call()
            .await
        {
            Ok(user) => Ok(Response::new(RegisterResponse {
                user_uuid: user.uuid.to_string(),
            })),
            Err(err) => Err(PresentationError::from(err).into()),
        }
    }

    async fn verify_user(
        &self,
        request: Request<VerifyUserRequest>,
    ) -> Result<Response<VerifyUserResponse>, Status> {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            self.config.rate_limiter.ip_address_header.as_str(),
            LimiterCategory::Auth,
        )
        .await
        .map_err(|status| *status)?;

        let request = request.into_inner();

        let user_uuid = self
            .verify_user_uc
            .execute(request.user_name, request.code)
            .await
            .map_err(PresentationError::from)?;

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
            self.config.rate_limiter.ip_address_header.as_str(),
            LimiterCategory::Auth,
        )
        .await
        .map_err(|status| *status)?;

        let user = self
            .resend_verification_uc
            .execute(request.into_inner().user_name)
            .await
            .map_err(PresentationError::from)?;

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
            self.config.rate_limiter.ip_address_header.as_str(),
            LimiterCategory::Auth,
        )
        .await
        .map_err(|status| *status)?;

        let request = request.into_inner();

        let jwt = match self
            .login_uc
            .execute(request.user_name, request.user_password)
            .await
        {
            Ok(user) => user,
            Err(err) => {
                return Err(PresentationError::from(err).into());
            }
        };

        Ok(Response::new(LoginResponse { jwt }))
    }

    async fn get_current_user(
        &self,
        request: Request<GetCurrentUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            self.config.rate_limiter.ip_address_header.as_str(),
            LimiterCategory::Auth,
        )
        .await
        .map_err(|status| *status)?;

        let req_user_uuid = match uuid_from_metadata(request.metadata()) {
            Ok(uuid) => uuid,
            Err(status) => return Err(*status),
        };

        let user = match self
            .get_user_uc
            .execute()
            .req_user_uuid(req_user_uuid)
            .user_uuid(req_user_uuid)
            .call()
            .await
        {
            Ok(user) => user,
            Err(err) => {
                return Err(PresentationError::from(err).into());
            }
        };

        Ok(Response::new(GetUserResponse {
            user_uuid: user.uuid.to_string(),
            user_name: user.name.into_inner(),
            email: user.email.into_inner(),
            user_role: user.role.into(),
            status: user.status.to_string(),
        }))
    }
}
