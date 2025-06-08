use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the specific outcome of processing a domain action,
/// including data needed for the final response and routing.
#[derive(Debug, Clone, Serialize, Deserialize)] // Serializable for Redis store
pub enum DomainActionOutcome {
    CorporationCreated {
        request_uuid: Uuid,
        tick_effective: i64,
        req_user_uuid: Uuid,
        corporation_uuid: Uuid,
        user_uuid: Uuid,
        corporation_name: String,
        corporation_balance: i64,
    },
    CorporationDeleted {
        request_uuid: Uuid,
        tick_effective: i64,
        req_user_uuid: Uuid,
        user_uuid: Uuid,
        corporation_uuid: Uuid,
    },
    ListedBusinessAcquired {
        request_uuid: Uuid,
        tick_effective: i64,
        req_user_uuid: Uuid,
        business_uuid: Uuid,
        market_uuid: Uuid,
        owning_corporation_uuid: Uuid,
        name: String,
        operational_expenses: i64,
    },
    UnitSpawned {
        request_uuid: Uuid,
        tick_effective: i64,
        req_user_uuid: Uuid,
        corporation_uuid: Uuid,
        unit_uuid: Uuid,
    },
    /// Failure Cases (Reportable failures)
    ActionFailed {
        request_uuid: Uuid,
        req_user_uuid: Uuid,
        tick_processed: i64,
        reason: String,
    },
}

impl DomainActionOutcome {
    pub fn get_req_user_uuid(&self) -> Uuid {
        match self {
            DomainActionOutcome::CorporationCreated { req_user_uuid, .. } => *req_user_uuid,
            DomainActionOutcome::CorporationDeleted { req_user_uuid, .. } => *req_user_uuid,
            DomainActionOutcome::ListedBusinessAcquired { req_user_uuid, .. } => *req_user_uuid,
            DomainActionOutcome::UnitSpawned { req_user_uuid, .. } => *req_user_uuid,
            DomainActionOutcome::ActionFailed { req_user_uuid, .. } => *req_user_uuid,
        }
    }

    pub fn get_request_uuid(&self) -> Uuid {
        match self {
            DomainActionOutcome::CorporationCreated { request_uuid, .. } => *request_uuid,
            DomainActionOutcome::CorporationDeleted { request_uuid, .. } => *request_uuid,
            DomainActionOutcome::ListedBusinessAcquired { request_uuid, .. } => *request_uuid,
            DomainActionOutcome::UnitSpawned { request_uuid, .. } => *request_uuid,
            DomainActionOutcome::ActionFailed { request_uuid, .. } => *request_uuid,
        }
    }
}
