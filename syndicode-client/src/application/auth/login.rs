use crate::domain::auth::repository::{AuthenticationRepository, LoginUserReq};
use bon::{bon, Builder};
use std::sync::Arc;
use syndicode_proto::syndicode_interface_v1::LoginResponse;
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
    ) -> anyhow::Result<LoginResponse> {
        let req = LoginUserReq {
            user_name,
            user_password,
        };

        self.auth_repository.lock().await.login_user(req).await
    }
}
