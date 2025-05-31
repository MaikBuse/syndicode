use std::sync::Arc;

use bon::{bon, Builder};
use tokio::sync::Mutex;

use crate::domain::{admin::AdminRepository, response::DomainResponse};

#[derive(Builder, Debug)]
pub struct GetUserUseCase<ADMIN>
where
    ADMIN: AdminRepository,
{
    admin_repository: Arc<Mutex<ADMIN>>,
}

#[bon]
impl<ADMIN> GetUserUseCase<ADMIN>
where
    ADMIN: AdminRepository,
{
    #[builder]
    pub async fn execute(
        &mut self,
        token: String,
        user_uuid: String,
    ) -> anyhow::Result<DomainResponse> {
        self.admin_repository
            .lock()
            .await
            .get_user(token, user_uuid)
            .await
    }
}
