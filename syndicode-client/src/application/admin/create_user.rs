use std::sync::Arc;

use bon::{bon, Builder};
use tokio::sync::Mutex;

use crate::domain::{
    admin::{AdminRepository, CreateUserDomainRequest},
    response::DomainResponse,
};

#[derive(Builder, Debug)]
pub struct CreateUserUseCase<ADMIN>
where
    ADMIN: AdminRepository,
{
    admin_repository: Arc<Mutex<ADMIN>>,
}

#[bon]
impl<ADMIN> CreateUserUseCase<ADMIN>
where
    ADMIN: AdminRepository,
{
    #[builder]
    pub async fn execute(
        &mut self,
        token: String,
        user_name: String,
        user_password: String,
        user_email: String,
        user_role: i32,
        corporation_name: String,
    ) -> anyhow::Result<DomainResponse> {
        let req = CreateUserDomainRequest {
            user_name,
            user_password,
            user_email,
            user_role,
            corporation_name,
        };

        self.admin_repository
            .lock()
            .await
            .create_user(token, req)
            .await
    }
}
