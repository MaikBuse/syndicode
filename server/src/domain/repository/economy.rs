use crate::domain::model::economy::CorporationModel;
use tonic::async_trait;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum EconomyDatabaseError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),
}

pub type EconomyDatabaseResult<T> = std::result::Result<T, EconomyDatabaseError>;

#[async_trait]
pub trait EconomyDatabaseRepository: std::fmt::Debug + Send + Sync {
    async fn create_corporation(
        &self,
        corporation: CorporationModel,
    ) -> EconomyDatabaseResult<CorporationModel>;

    async fn get_user_corporation(
        &self,
        session_uuid: Uuid,
        user_uuid: Uuid,
    ) -> EconomyDatabaseResult<CorporationModel>;

    async fn update_corporation(
        &self,
        corporation: CorporationModel,
    ) -> EconomyDatabaseResult<CorporationModel>;
}
