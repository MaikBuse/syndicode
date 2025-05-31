use super::response::DomainResponse;

pub struct CreateUserDomainRequest {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    pub user_role: i32,
    pub corporation_name: String,
}

pub trait AdminRepository {
    async fn create_user(
        &mut self,
        token: String,
        req: CreateUserDomainRequest,
    ) -> anyhow::Result<DomainResponse>;

    async fn get_user(
        &mut self,
        token: String,
        user_uuid: String,
    ) -> anyhow::Result<DomainResponse>;

    async fn delete_user(
        &mut self,
        token: String,
        user_uuid: String,
    ) -> anyhow::Result<DomainResponse>;
}
