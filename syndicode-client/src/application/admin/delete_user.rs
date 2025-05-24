use std::sync::Arc;

use bon::{bon, Builder};
use tokio::sync::Mutex;

use crate::domain::{admin::AdminRepository, response::DomainResponse};

#[derive(Builder, Debug)]
pub struct DeleteUserUseCase<ADMIN>
where
    ADMIN: AdminRepository,
{
    admin_repository: Arc<Mutex<ADMIN>>,
}

#[bon]
impl<ADMIN> DeleteUserUseCase<ADMIN>
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
            .delete_user(token, user_uuid)
            .await
    }
}
