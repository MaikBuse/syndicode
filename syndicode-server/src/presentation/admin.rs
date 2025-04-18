use super::common::{application_error_into_status, check_rate_limit, uuid_from_metadata};
use crate::{
    application::{
        admin::{create_user::CreateUserUseCase, delete_user::DeleteUserUseCase},
        ports::{
            crypto::PasswordHandler,
            limiter::{LimiterCategory, RateLimitEnforcer},
            uow::UnitOfWork,
        },
    },
    config::Config,
    domain::user::{model::role::UserRole, repository::UserRepository},
};
use std::{result::Result, sync::Arc};
use syndicode_proto::syndicode_interface_v1::{
    admin_service_server::AdminService, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, UserRole as ProtoUserRole,
};
use tonic::{async_trait, Request, Response, Status};
use uuid::Uuid;

pub struct AdminPresenter<R, P, UOW, USR>
where
    R: RateLimitEnforcer + 'static,
    P: PasswordHandler + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
{
    pub config: Arc<Config>,
    pub limit: Arc<R>,
    pub create_user_uc: Arc<CreateUserUseCase<P, UOW, USR>>,
    pub delete_user_uc: Arc<DeleteUserUseCase<USR>>,
}

#[async_trait]
impl<R, P, UOW, USR> AdminService for AdminPresenter<R, P, UOW, USR>
where
    R: RateLimitEnforcer + 'static,
    P: PasswordHandler + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
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
                    "The user's role needs to either be 'Player' or 'Admin'",
                ));
            }
            ProtoUserRole::Player => UserRole::Player,
            ProtoUserRole::Admin => UserRole::Admin,
        };

        match self
            .create_user_uc
            .execute(
                Some(req_user_uuid),
                request.user_name,
                request.user_password,
                user_role.clone(),
                request.corporation_name,
            )
            .await
        {
            Ok(user) => {
                let user_role: i32 = match user.role {
                    UserRole::Admin => 1,
                    UserRole::Player => 2,
                };

                Ok(Response::new(CreateUserResponse {
                    user_uuid: user.uuid.to_string(),
                    user_name: user.name,
                    user_role,
                }))
            }
            Err(err) => Err(application_error_into_status(err)),
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
            Err(err) => Err(application_error_into_status(err)),
        }
    }
}
