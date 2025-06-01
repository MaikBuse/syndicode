use syndicode_proto::syndicode_interface_v1::{
    CreateUserResponse, DeleteUserResponse, GetUserResponse,
};

pub struct CreateUserDomainRequest {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    pub user_role: i32,
    pub corporation_name: String,
}

#[tonic::async_trait]
pub trait AdminRepository {
    async fn create_user(
        &mut self,
        token: String,
        req: CreateUserDomainRequest,
    ) -> anyhow::Result<CreateUserResponse>;

    async fn get_user(
        &mut self,
        token: String,
        user_uuid: String,
    ) -> anyhow::Result<GetUserResponse>;

    async fn delete_user(
        &mut self,
        token: String,
        user_uuid: String,
    ) -> anyhow::Result<DeleteUserResponse>;
}
