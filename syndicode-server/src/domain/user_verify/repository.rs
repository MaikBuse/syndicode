use super::model::UserVerification;
use crate::domain::repository::RepositoryResult;
use tonic::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserVerificationTxRepository: Send + Sync {
    async fn create_user_verification(
        &mut self,
        user_verification: &UserVerification,
    ) -> RepositoryResult<()>;
    async fn get_user_verification(
        &mut self,
        user_uuid: Uuid,
    ) -> RepositoryResult<UserVerification>;
    async fn update_user_verification(
        &mut self,
        user_verification: &UserVerification,
    ) -> RepositoryResult<()>;
    async fn delete_user_verification(&mut self, user_uuid: Uuid) -> RepositoryResult<()>;
}
