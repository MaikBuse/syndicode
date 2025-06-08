use syndicode_proto::syndicode_interface_v1::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse, GetUserRequest,
    GetUserResponse,
};
use tonic::Request;

use crate::domain::admin::{AdminRepository, CreateUserDomainRequest, DeleteUserDomainRequest};

use super::grpc::GrpcHandler;

#[tonic::async_trait]
impl AdminRepository for GrpcHandler {
    async fn create_user(
        &mut self,
        token: String,
        req: CreateUserDomainRequest,
    ) -> anyhow::Result<CreateUserResponse> {
        let mut request = Request::new(CreateUserRequest {
            request_uuid: req.request_uuid,
            user_name: req.user_name,
            user_password: req.user_password,
            user_email: req.user_email,
            user_role: req.user_role,
            corporation_name: req.corporation_name,
        });

        self.add_ip_metadata(request.metadata_mut())?;
        self.add_token_metadata(request.metadata_mut(), token)?;

        Ok(self
            .admin_client
            .create_user(request)
            .await
            .map_err(|status| anyhow::anyhow!("{}", status))?
            .into_inner())
    }

    async fn get_user(
        &mut self,
        token: String,
        user_uuid: String,
    ) -> anyhow::Result<GetUserResponse> {
        let mut request = Request::new(GetUserRequest { user_uuid });

        self.add_ip_metadata(request.metadata_mut())?;
        self.add_token_metadata(request.metadata_mut(), token)?;

        Ok(self.admin_client.get_user(request).await?.into_inner())
    }

    async fn delete_user(
        &mut self,
        token: String,
        req: DeleteUserDomainRequest,
    ) -> anyhow::Result<DeleteUserResponse> {
        let mut request = Request::new(DeleteUserRequest {
            request_uuid: req.request_uuid,
            user_uuid: req.user_uuid,
        });

        self.add_ip_metadata(request.metadata_mut())?;
        self.add_token_metadata(request.metadata_mut(), token)?;

        Ok(self
            .admin_client
            .delete_user(request)
            .await
            .map_err(|status| anyhow::anyhow!("{}", status))?
            .into_inner())
    }
}
