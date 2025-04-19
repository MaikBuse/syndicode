use crate::domain::corporation::model::Corporation;
use bon::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Builder, serde::Serialize, serde::Deserialize)]
pub struct QueuedActionPayload {
    pub user_uuid: Uuid,
    pub details: ActionDetails,
    pub request_uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub enum ActionDetails {
    SpawnUnit { req_user_uuid: Uuid },
    UpdateCorporation { corporation: Corporation },
}
