use std::fmt::Display;

use bon::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::economy::corporation::model::name::CorporationName;

#[derive(Builder, serde::Serialize, serde::Deserialize)]
pub struct QueuedAction {
    pub id: String,
    pub payload: QueuedActionPayload,
}

#[derive(Builder, serde::Serialize, serde::Deserialize)]
pub struct QueuedActionPayload {
    pub request_uuid: Uuid,
    pub req_user_uuid: Uuid,
    pub details: ActionDetails,
}

#[derive(Serialize, Deserialize)]
pub enum ActionDetails {
    CreateCorporation {
        user_uuid: Uuid,
        corporation_name: CorporationName,
    },
    DeleteCorporation {
        corporation_uuid: Uuid,
    },
    SpawnUnit,
    AcquireListedBusiness {
        business_listing_uuid: Uuid,
    },
}

impl ActionDetails {
    pub fn get_order(&self) -> u16 {
        match self {
            ActionDetails::CreateCorporation { .. } => 1,
            ActionDetails::SpawnUnit => 2,
            ActionDetails::AcquireListedBusiness { .. } => 3,
            ActionDetails::DeleteCorporation { .. } => 4,
        }
    }
}

impl Display for ActionDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionDetails::CreateCorporation { .. } => write!(f, "CreateCorporation"),
            ActionDetails::DeleteCorporation { .. } => write!(f, "DeleteCorporation"),
            ActionDetails::SpawnUnit => write!(f, "SpawnUnit"),
            ActionDetails::AcquireListedBusiness { .. } => write!(f, "AcquireListedBusiness"),
        }
    }
}
