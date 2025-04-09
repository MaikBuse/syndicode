use crate::domain::corporation::model::Corporation;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum QueuedAction {
    SpawnUnit { req_user_uuid: Uuid },
    UpdateCorporation { corporation: Corporation },
}
