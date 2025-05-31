use super::{
    common::{check_rate_limit, parse_uuid, uuid_from_metadata},
    error::PresentationError,
};
use crate::{
    application::{
        admin::get_user::GetUserUseCase,
        admin::{create_user::CreateUserUseCase, delete_user::DeleteUserUseCase},
        ports::{
            crypto::PasswordHandler,
            limiter::{LimiterCategory, RateLimitEnforcer},
            uow::UnitOfWork,
            verification::VerificationSendable,
        },
    },
    config::Config,
    domain::user::{model::role::UserRole, repository::UserRepository},
};
use bon::Builder;
use std::{result::Result, sync::Arc};
use syndicode_proto::syndicode_interface_v1::{
    admin_service_server::AdminService, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, GetUserRequest, GetUserResponse, UserRole as ProtoUserRole,
};
use tonic::{async_trait, Request, Response, Status};
use uuid::Uuid;

#[derive(Builder)]
pub struct AdminPresenter<R, P, UOW, USR, VS>
where
    R: RateLimitEnforcer + 'static,
    P: PasswordHandler + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
    VS: VerificationSendable + 'static,
{
    config: Arc<Config>,
    limit: Arc<R>,
    create_user_uc: Arc<CreateUserUseCase<P, UOW, USR, VS>>,
    get_user_uc: Arc<GetUserUseCase<USR>>,
    delete_user_uc: Arc<DeleteUserUseCase<USR>>,
}

#[async_trait]
impl<R, P, UOW, USR, VS> AdminService for AdminPresenter<R, P, UOW, USR, VS>
where
    R: RateLimitEnforcer + 'static,
    P: PasswordHandler + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
    VS: VerificationSendable + 'static,
{
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            &self.config.ip_address_header,
            LimiterCategory::Admin,
        )
        .await?;

        let req_user_uuid = match uuid_from_metadata(request.metadata()) {
            Ok(uuid) => uuid,
            Err(status) => return Err(status),
        };

        let request = request.into_inner();

        let user_role = match request.user_role() {
            ProtoUserRole::Unspecified => {
                return Err(Status::invalid_argument(
                    "The user's role needs to either be '1' (Admin) or '2' (Player)",
                ));
            }
            ProtoUserRole::Player => UserRole::Player,
            ProtoUserRole::Admin => UserRole::Admin,
        };

        match self
            .create_user_uc
            .execute()
            .maybe_req_user_uuid(req_user_uuid)
            .user_name(request.user_name)
            .password(request.user_password)
            .user_email(request.user_email)
            .user_role(user_role)
            .corporation_name(request.corporation_name)
            .call()
            .await
        {
            Ok(user) => {
                let user_role: i32 = match user.role {
                    UserRole::Admin => 1,
                    UserRole::Player => 2,
                };

                Ok(Response::new(CreateUserResponse {
                    user_uuid: user.uuid.to_string(),
                    user_name: user.name.into_inner(),
                    user_role,
                }))
            }
            Err(err) => Err(PresentationError::from(err).into()),
        }
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            &self.config.ip_address_header,
            LimiterCategory::Admin,
        )
        .await?;

        let req_user_uuid = match uuid_from_metadata(request.metadata()) {
            Ok(uuid) => uuid,
            Err(status) => return Err(status),
        };

        let Ok(user_uuid) = Uuid::parse_str(request.into_inner().user_uuid.as_str()) else {
            return Err(Status::invalid_argument("Failed to parse user uuid"));
        };

        match self.delete_user_uc.execute(req_user_uuid, user_uuid).await {
            Ok(_) => Ok(Response::new(DeleteUserResponse {})),
            Err(err) => Err(PresentationError::from(err).into()),
        }
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        check_rate_limit(
            self.limit.clone(),
            request.metadata(),
            &self.config.ip_address_header,
            LimiterCategory::Auth,
        )
        .await?;

        let req_user_uuid = match uuid_from_metadata(request.metadata()) {
            Ok(uuid) => uuid,
            Err(status) => return Err(status),
        };

        let request = request.into_inner();

        let user_uuid = parse_uuid(request.user_uuid.as_str())?;

        let user = match self
            .get_user_uc
            .execute()
            .req_user_uuid(req_user_uuid)
            .user_uuid(user_uuid)
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
