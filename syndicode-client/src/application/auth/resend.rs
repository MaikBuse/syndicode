use crate::domain::{
    auth::{AuthenticationRepository, ResendVerificationReq},
    response::DomainResponse,
};
use bon::{bon, Builder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Builder, Debug)]
pub struct ResendVerificationUseCase<AUTH>
where
    AUTH: AuthenticationRepository,
{
    auth_repository: Arc<Mutex<AUTH>>,
}

#[bon]
impl<AUTH> ResendVerificationUseCase<AUTH>
where
    AUTH: AuthenticationRepository,
{
    #[builder]
    pub async fn execute(&mut self, user_name: String) -> anyhow::Result<DomainResponse> {
        let req = ResendVerificationReq { user_name };

        self.auth_repository
            .lock()
            .await
            .resend_verification(req)
            .await
    }
}
