use super::common::{application_error_into_status, uuid_from_metadata};
use crate::{
    application::{
        admin::{create_user::CreateUserUseCase, delete_user::DeleteUserUseCase},
        ports::{crypto::PasswordHandler, uow::UnitOfWork},
    },
    domain::user::{model::role::UserRole, repository::UserRepository},
};
use std::{result::Result, sync::Arc};
use syndicode_proto::syndicode_interface_v1::{
    admin_service_server::AdminService, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, UserRole as ProtoUserRole,
};
use tonic::{async_trait, Request, Response, Status};
use uuid::Uuid;

pub struct AdminPresenter<P, UOW, USR>
where
    P: PasswordHandler + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
{
    pub create_user_uc: Arc<CreateUserUseCase<P, UOW, USR>>,
    pub delete_user_uc: Arc<DeleteUserUseCase<USR>>,
}

#[async_trait]
impl<P, UOW, USR> AdminService for AdminPresenter<P, UOW, USR>
where
    P: PasswordHandler + 'static,
    UOW: UnitOfWork + 'static,
    USR: UserRepository + 'static,
{
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req_user_uuid = match uuid_from_metadata(request.metadata()) {
            Ok(uuid) => uuid,
            Err(status) => return Err(status),
        };

        let request = request.into_inner();

        let user_role = match request.user_role() {
            ProtoUserRole::Unspecified => {
                return Err(Status::invalid_argument(
                    "The user role needs to either be 'User' or 'Admin'",
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
