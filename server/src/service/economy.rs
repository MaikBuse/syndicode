use crate::domain::{
    model::economy::CorporationModel,
    repository::{
        control::{ControlDatabaseError, ControlDatabaseRepository},
        economy::{EconomyDatabaseError, EconomyDatabaseRepository},
    },
};
use std::sync::Arc;
use tokio::sync::Mutex;

type Result<T> = std::result::Result<T, EconomyServiceError>;

#[derive(Debug, thiserror::Error)]
pub enum EconomyServiceError {
    #[error(transparent)]
    ControlDatabase(#[from] ControlDatabaseError),

    #[error(transparent)]
    EconomyDatabase(#[from] EconomyDatabaseError),
}

pub type EconomyServiceResult<T> = std::result::Result<T, EconomyServiceError>;

#[derive(Debug)]
pub struct EconomyService {
    control_db: Arc<Mutex<dyn ControlDatabaseRepository>>,
    economy_db: Arc<Mutex<dyn EconomyDatabaseRepository>>,
}

impl EconomyService {
    pub fn new(
        control_db: Arc<Mutex<dyn ControlDatabaseRepository>>,
        economy_db: Arc<Mutex<dyn EconomyDatabaseRepository>>,
    ) -> Self {
        Self {
            control_db,
            economy_db,
        }
    }

    pub async fn get_corporation(
        &self,
        session_uuid: Vec<u8>,
        user_uuid: Vec<u8>,
    ) -> EconomyServiceResult<CorporationModel> {
        let economy_db = self.economy_db.lock().await;

        Ok(economy_db
            .get_user_corporation(session_uuid, user_uuid)
            .await?)
    }
}
