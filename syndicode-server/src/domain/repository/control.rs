use crate::domain::model::control::UserModel;
use tonic::async_trait;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ControlDatabaseError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),

    #[error("The database returned with a violation of a unique/primary key constraint")]
    UniqueConstraint,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ControlDatabaseResult<T> = std::result::Result<T, ControlDatabaseError>;

#[async_trait]
pub trait ControlDatabaseRepository: std::fmt::Debug + Send + Sync {
    async fn create_user(&self, user: UserModel) -> ControlDatabaseResult<UserModel>;
    async fn get_user(&self, user_uuid: Uuid) -> ControlDatabaseResult<UserModel>;
    async fn get_user_by_name(&self, username: String) -> ControlDatabaseResult<UserModel>;
    async fn delete_user(&self, user_uuid: Uuid) -> ControlDatabaseResult<()>;
}
