use crate::domain::{
    model::warfare::UnitModel,
    repository::{
        control::{ControlDatabaseError, ControlDatabaseRepository},
        warfare::{WarfareDatabaseError, WarfareDatabaseRepository},
    },
};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

type Result<T> = std::result::Result<T, WarfareServiceError>;

#[derive(Debug, thiserror::Error)]
pub enum WarfareServiceError {
    #[error(transparent)]
    ControlDatabase(#[from] ControlDatabaseError),

    #[error(transparent)]
    WarfareDatabase(#[from] WarfareDatabaseError),
}

pub type WarfareServiceResult<T> = std::result::Result<T, WarfareServiceError>;

#[derive(Debug)]
pub struct WarfareService {
    control_db: Arc<Mutex<dyn ControlDatabaseRepository>>,
    warfare_db: Arc<Mutex<dyn WarfareDatabaseRepository>>,
}

impl WarfareService {
    pub fn new(
        control_db: Arc<Mutex<dyn ControlDatabaseRepository>>,
        warfare_db: Arc<Mutex<dyn WarfareDatabaseRepository>>,
    ) -> Self {
        Self {
            control_db,
            warfare_db,
        }
    }

    pub async fn create_unit(
        &self,
        session_uuid: Vec<u8>,
        user_uuid: Vec<u8>,
    ) -> WarfareServiceResult<UnitModel> {
        let warfare_db = self.warfare_db.lock().await;

        let unit = UnitModel {
            uuid: Uuid::now_v7().into(),
            session_uuid,
            user_uuid,
        };

        Ok(warfare_db.create_unit(unit).await?)
    }

    pub async fn list_units(
        &self,
        session_uuid: Vec<u8>,
        user_uuid: Vec<u8>,
    ) -> Result<Vec<UnitModel>> {
        let warfare_db = self.warfare_db.lock().await;

        let units = warfare_db.list_user_units(session_uuid, user_uuid).await?;

        Ok(units)
    }
}
