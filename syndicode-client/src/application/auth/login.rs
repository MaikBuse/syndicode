use crate::domain::{
    auth::{AuthenticationRepository, LoginUserReq},
    response::DomainResponse,
};
use bon::{bon, Builder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Builder, Debug)]
pub struct LoginUserUseCase<AUTH>
where
    AUTH: AuthenticationRepository,
{
    auth_repository: Arc<Mutex<AUTH>>,
}

#[bon]
impl<AUTH> LoginUserUseCase<AUTH>
where
    AUTH: AuthenticationRepository,
{
    #[builder]
    pub async fn execute(
        &mut self,
        user_name: String,
        user_password: String,
    ) -> anyhow::Result<(String, DomainResponse)> {
        let req = LoginUserReq {
            user_name,
            user_password,
        };

        self.auth_repository.lock().await.login_user(req).await
    }
}
