use std::sync::Arc;

use bon::{bon, Builder};
use syndicode_proto::syndicode_interface_v1::GetUserResponse;
use tokio::sync::Mutex;

use crate::domain::auth::repository::AuthenticationRepository;

#[derive(Builder, Debug)]
pub struct GetCurrentUserUseCase<AUTH>
where
    AUTH: AuthenticationRepository,
{
    auth_repository: Arc<Mutex<AUTH>>,
}

#[bon]
impl<AUTH> GetCurrentUserUseCase<AUTH>
where
    AUTH: AuthenticationRepository,
{
    #[builder]
    pub async fn execute(&mut self, token: String) -> anyhow::Result<GetUserResponse> {
        self.auth_repository
            .lock()
            .await
            .get_current_user(token)
            .await
    }
}
