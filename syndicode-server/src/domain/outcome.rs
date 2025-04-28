use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the specific outcome of processing a domain action,
/// including data needed for the final response and routing.
#[derive(Debug, Clone, Serialize, Deserialize)] // Serializable for Redis store
pub enum DomainActionOutcome {
    ListedBusinessAcquired {
        request_uuid: Uuid,
        tick_effective: i64,
        user_uuid: Uuid,
        business_uuid: Uuid,
        market_uuid: Uuid,
        owning_corporation_uuid: Uuid,
        name: String,
        operational_expenses: i64,
    },
    UnitSpawned {
        request_uuid: Uuid,
        tick_effective: i64,
        /// Target client
        user_uuid: Uuid,
        /// Result data: ID of the newly spawned unit
        unit_uuid: Uuid,
    },
    /// Failure Cases (Reportable failures)
    ActionFailed {
        request_uuid: Uuid,
        tick_processed: i64,
        user_uuid: Uuid,
        reason: String,
    },
}
