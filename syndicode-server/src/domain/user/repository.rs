#[cfg(test)]
use mockall::{automock, predicate::*};

use super::model::User;
use crate::domain::repository::RepositoryResult;
use tonic::async_trait;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &User) -> RepositoryResult<()>;
    async fn get_user(&self, user_uuid: Uuid) -> RepositoryResult<User>;
    async fn get_user_by_name(&self, user_name: String) -> RepositoryResult<User>;
    async fn delete_user(&self, user_uuid: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
pub trait UserTxRepository: Send + Sync {
    async fn create_user(&mut self, user: &User) -> RepositoryResult<()>;
    async fn get_user(&mut self, user_uuid: Uuid) -> RepositoryResult<User>;
    async fn get_user_by_name(&mut self, user_name: String) -> RepositoryResult<User>;
    async fn update_user(&mut self, user: &User) -> RepositoryResult<()>;
    async fn delete_user(&mut self, user_uuid: Uuid) -> RepositoryResult<()>;
}
