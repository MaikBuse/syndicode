use crate::domain::model::control::UserModel;
use sqlx::{Postgres, Transaction};
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

enum ExecutorEnum {
    Transaction,
    Pool,
}

pub type ControlDatabaseResult<T> = std::result::Result<T, ControlDatabaseError>;

#[async_trait]
pub trait ControlDatabaseRepository: std::fmt::Debug + Send + Sync {
    async fn create_user<'e, E>(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user: UserModel,
    ) -> ControlDatabaseResult<UserModel>
    where
        E: sqlx::Executor<'e, Database = Postgres> + Send;
    async fn get_user(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_uuid: Uuid,
    ) -> ControlDatabaseResult<UserModel>;
    async fn get_user_by_name(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        username: String,
    ) -> ControlDatabaseResult<UserModel>;
    async fn delete_user(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_uuid: Uuid,
    ) -> ControlDatabaseResult<()>;
}
