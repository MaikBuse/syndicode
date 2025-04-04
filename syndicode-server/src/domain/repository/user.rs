use super::RepositoryResult;
use crate::domain::user::User;
use tonic::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user(&self, user_uuid: Uuid) -> RepositoryResult<User>;
    async fn get_user_by_name(&self, user_name: String) -> RepositoryResult<User>;
    async fn create_user(&self, user: User) -> RepositoryResult<User>;
    async fn delete_user(&self, user_uuid: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
pub trait UserTxRepository: Send + Sync {
    async fn get_user(&mut self, user_uuid: Uuid) -> RepositoryResult<User>;
    async fn get_user_by_name(&mut self, user_name: String) -> RepositoryResult<User>;
    async fn create_user(&mut self, user: User) -> RepositoryResult<User>;
    async fn delete_user(&mut self, user_uuid: Uuid) -> RepositoryResult<()>;
}
