use std::sync::Arc;

use bon::{bon, Builder};
use syndicode_proto::syndicode_interface_v1::GetUserResponse;
use tokio::sync::Mutex;

use crate::domain::admin::AdminRepository;

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
    ) -> anyhow::Result<GetUserResponse> {
        self.admin_repository
            .lock()
            .await
            .get_user(token, user_uuid)
            .await
    }
}
