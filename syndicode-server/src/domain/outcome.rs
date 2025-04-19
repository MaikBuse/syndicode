use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the specific outcome of processing a domain action,
/// including data needed for the final response and routing.
#[derive(Debug, Clone, Serialize, Deserialize)] // Serializable for Redis store
pub enum DomainActionOutcome {
    UnitSpawned {
        /// Target client
        user_uuid: Uuid,
        request_uuid: Uuid,
        /// Result data: ID of the newly spawned unit
        unit_uuid: Uuid,
        tick_effective: i64,
    },
    /// Failure Cases (Reportable failures)
    ActionFailed {
        user_uuid: Uuid,
        request_uuid: Uuid,
        reason: String,
        tick_processed: i64,
    },
}

impl DomainActionOutcome {
    pub fn player_id(&self) -> Uuid {
        match self {
            DomainActionOutcome::UnitSpawned {
                user_uuid: player_id,
                ..
            } => *player_id,
            DomainActionOutcome::ActionFailed {
                user_uuid: player_id,
                ..
            } => *player_id,
        }
    }

    pub fn request_uuid(&self) -> &Uuid {
        match self {
            DomainActionOutcome::UnitSpawned { request_uuid, .. } => request_uuid,
            DomainActionOutcome::ActionFailed { request_uuid, .. } => request_uuid,
        }
    }
}
