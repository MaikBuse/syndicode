use std::sync::Arc;

use bon::{bon, Builder};
use tokio::sync::Mutex;

use crate::domain::{auth::AuthenticationRepository, response::DomainResponse};

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
    pub async fn execute(&mut self, token: String) -> anyhow::Result<DomainResponse> {
        self.auth_repository
            .lock()
            .await
            .get_current_user(token)
            .await
    }
}
