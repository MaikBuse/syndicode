use crate::domain::{
    auth::{AuthenticationRepository, RegisterUserReq},
    response::Response,
};
use bon::{bon, Builder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Builder, Debug)]
pub struct RegisterUseCase<AUTH>
where
    AUTH: AuthenticationRepository,
{
    auth_repository: Arc<Mutex<AUTH>>,
}

#[bon]
impl<AUTH> RegisterUseCase<AUTH>
where
    AUTH: AuthenticationRepository,
{
    #[builder]
    pub async fn execute(
        &mut self,
        user_name: String,
        user_password: String,
        email: String,
        corporation_name: String,
    ) -> anyhow::Result<Response> {
        let req = RegisterUserReq::builder()
            .user_name(user_name)
            .user_password(user_password)
            .email(email)
            .corporation_name(corporation_name)
            .build();

        let mut repo = self.auth_repository.lock().await;

        repo.register_user(req).await
    }
}
