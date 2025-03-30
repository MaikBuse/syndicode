use crate::domain::model::warfare::UnitModel;
use sqlx::{Postgres, Transaction};
use tonic::async_trait;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum WarfareDatabaseError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),
}

pub type WarfareDatabaseResult<T> = std::result::Result<T, WarfareDatabaseError>;

#[async_trait]
pub trait WarfareDatabaseRepository: std::fmt::Debug + Send + Sync {
    async fn create_unit(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        unit: UnitModel,
    ) -> WarfareDatabaseResult<UnitModel>;

    async fn list_user_units(
        &self,

        tx: &mut Transaction<'_, Postgres>,
        user_uuid: Uuid,
    ) -> WarfareDatabaseResult<Vec<UnitModel>>;
}
