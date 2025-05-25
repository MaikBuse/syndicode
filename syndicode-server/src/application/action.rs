use std::fmt::Display;

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
    SpawnUnit,
    AcquireListedBusiness { business_listing_uuid: Uuid },
}

impl Display for ActionDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionDetails::SpawnUnit => write!(f, "SpawnUnit"),
            ActionDetails::AcquireListedBusiness { .. } => write!(f, "AcquireListedBusiness"),
        }
    }
}
