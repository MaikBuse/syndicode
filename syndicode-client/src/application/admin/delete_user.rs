use std::sync::Arc;

use bon::{bon, Builder};
use syndicode_proto::syndicode_interface_v1::DeleteUserResponse;
use tokio::sync::Mutex;

use crate::domain::admin::{AdminRepository, DeleteUserDomainRequest};

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
        request_uuid: String,
        user_uuid: String,
    ) -> anyhow::Result<DeleteUserResponse> {
        let req = DeleteUserDomainRequest {
            request_uuid,
            user_uuid,
        };

        self.admin_repository
            .lock()
            .await
            .delete_user(token, req)
            .await
    }
}
