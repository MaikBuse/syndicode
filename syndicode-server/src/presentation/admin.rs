use super::common::{service_error_into_status, uuid_from_metadata};
use crate::{domain::model::interface::UserRole, service::interface::InterfaceService};
use std::{result::Result, sync::Arc};
use syndicode_proto::syndicode_interface_v1::{
    admin_service_server::AdminService, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, UserRole as ProtoUserRole,
};
use tonic::{async_trait, Request, Response, Status};
use uuid::Uuid;

pub struct AdminPresenter {
    pub interface_service: Arc<InterfaceService>,
}

#[async_trait]
impl AdminService for AdminPresenter {
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
            .interface_service
            .create_user(
                Some(req_user_uuid),
                request.user_name,
                request.user_password,
                user_role.clone(),
                request.corporation_name,
            )
            .await
        {
            Ok(user) => Ok(Response::new(CreateUserResponse {
                user_uuid: user.uuid.to_string(),
                user_name: user.name,
                user_role: user_role.into(),
            })),
            Err(err) => Err(service_error_into_status(err)),
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

        match self
            .interface_service
            .delete_user(req_user_uuid, user_uuid)
            .await
        {
            Ok(_) => Ok(Response::new(DeleteUserResponse {})),
            Err(err) => Err(service_error_into_status(err)),
        }
    }
}
