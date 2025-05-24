use crate::domain::response::DomainResponse;
use bon::Builder;

#[derive(Builder)]
pub struct RegisterUserReq {
    pub user_name: String,
    pub user_password: String,
    pub email: String,
    pub corporation_name: String,
}

pub struct VerifyUserReq {
    pub user_name: String,
    pub code: String,
}

pub struct ResendVerificationReq {
    pub user_name: String,
}

pub struct LoginUserReq {
    pub user_name: String,
    pub user_password: String,
}

pub trait AuthenticationRepository {
    async fn register_user(&mut self, req: RegisterUserReq) -> anyhow::Result<DomainResponse>;
    async fn verifiy_user(&mut self, req: VerifyUserReq) -> anyhow::Result<DomainResponse>;
    async fn resend_verification(
        &mut self,
        req: ResendVerificationReq,
    ) -> anyhow::Result<DomainResponse>;
    async fn login_user(&mut self, req: LoginUserReq) -> anyhow::Result<(String, DomainResponse)>;
}
