use syndicode_proto::syndicode_interface_v1::{
    CreateUserRequest, DeleteUserRequest, GetUserRequest,
};
use tonic::Request;

use crate::domain::{
    admin::{AdminRepository, CreateUserDomainRequest},
    response::DomainResponse,
};

use super::grpc::GrpcHandler;

#[tonic::async_trait]
impl AdminRepository for GrpcHandler {
    async fn create_user(
        &mut self,
        token: String,
        req: CreateUserDomainRequest,
    ) -> anyhow::Result<DomainResponse> {
        let mut request = Request::new(CreateUserRequest {
            user_name: req.user_name,
            user_password: req.user_password,
            user_email: req.user_email,
            user_role: req.user_role,
            corporation_name: req.corporation_name,
        });

        self.add_ip_metadata(request.metadata_mut())?;
        self.add_token_metadata(request.metadata_mut(), token)?;

        let result = self.admin_client.create_user(request).await;

        self.response_from_result(result)
    }

    async fn delete_user(
        &mut self,
        token: String,
        user_uuid: String,
    ) -> anyhow::Result<DomainResponse> {
        let mut request = Request::new(DeleteUserRequest { user_uuid });

        self.add_ip_metadata(request.metadata_mut())?;
        self.add_token_metadata(request.metadata_mut(), token)?;

        let result = self.admin_client.delete_user(request).await;

        self.response_from_result(result)
    }

    async fn get_user(
        &mut self,
        token: String,
        user_uuid: String,
    ) -> anyhow::Result<DomainResponse> {
        let mut request = Request::new(GetUserRequest { user_uuid });

        self.add_ip_metadata(request.metadata_mut())?;
        self.add_token_metadata(request.metadata_mut(), token)?;

        let result = self.admin_client.get_user(request).await;

        self.response_from_result(result)
    }
}
