use crate::domain::{
    auth::{AuthenticationRepository, VerifyUserReq},
    response::Response,
};
use bon::{bon, Builder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Builder, Debug)]
pub struct VerifyUserUseCase<AUTH>
where
    AUTH: AuthenticationRepository,
{
    auth_repository: Arc<Mutex<AUTH>>,
}

#[bon]
impl<AUTH> VerifyUserUseCase<AUTH>
where
    AUTH: AuthenticationRepository,
{
    #[builder]
    pub async fn execute(&mut self, user_name: String, code: String) -> anyhow::Result<Response> {
        let req = VerifyUserReq { user_name, code };

        self.auth_repository.lock().await.verifiy_user(req).await
    }
}
