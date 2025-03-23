use crate::domain::model::control::{SessionModel, SessionUser, UserModel};
use tonic::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum ControlDatabaseError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),
}

pub type ControlDatabaseResult<T> = std::result::Result<T, ControlDatabaseError>;

#[async_trait]
pub trait ControlDatabaseRepository: std::fmt::Debug + Send + Sync {
    async fn create_user(&self, user: UserModel) -> ControlDatabaseResult<UserModel>;
    async fn get_user(&self, user_uuid: Vec<u8>) -> ControlDatabaseResult<UserModel>;
    async fn get_user_by_name(&self, username: String) -> ControlDatabaseResult<UserModel>;
    async fn delete_user(&self, user_uuid: Vec<u8>) -> ControlDatabaseResult<()>;
    async fn create_session(&self, session_uuid: Vec<u8>) -> ControlDatabaseResult<SessionModel>;
    async fn get_session(&self, session_uuid: Vec<u8>) -> ControlDatabaseResult<SessionModel>;
    async fn list_sessions(&self) -> ControlDatabaseResult<Vec<SessionModel>>;
    async fn update_session(&self, session: SessionModel) -> ControlDatabaseResult<SessionModel>;
    async fn delete_session(&self, session_uuid: Vec<u8>) -> ControlDatabaseResult<()>;
    async fn create_session_user(
        &self,
        session_user: SessionUser,
    ) -> ControlDatabaseResult<SessionUser>;
    async fn get_session_user(
        &self,
        session_uuid: Vec<u8>,
        user_uuid: Vec<u8>,
    ) -> ControlDatabaseResult<SessionUser>;
}
