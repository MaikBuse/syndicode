pub mod game_state;
mod handlers;
mod saga;

use super::{outcome::DomainActionOutcome, ports::simulation::Simulationable};
use crate::application::action::{ActionDetails, QueuedActionPayload};
use bon::builder;
use game_state::GameState;
use handlers::{
    acquire_listed_business::handle_acquire_listed_business, spawn_unit::handle_spawn_unit,
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, PartialEq)]
pub enum ActionError {
    #[error("User '{user_uuid}' not associated with any corporation.")]
    RequestingCorporationNotFoundByUser { user_uuid: Uuid },

    #[error("Business listing '{listing_uuid}' not found.")]
    BusinessListingNotFound { listing_uuid: Uuid },

    #[error("Corporation '{corporation_uuid}' required for checks phase not found.")]
    CorporationNotFoundDuringChecks { corporation_uuid: Uuid },

    #[error("Business '{business_uuid}' required for checks phase not found.")]
    BusinessNotFoundDuringChecks { business_uuid: Uuid },

    #[error("Corporation '{corporation_uuid}' has insufficient funds. Required: {required}, Available: {available}.")]
    InsufficientFunds {
        corporation_uuid: Uuid,
        required: i64,
        available: i64,
    },

    #[error("Saga execution failed during step '{step_description}': Entity '{entity_type}' with ID '{entity_id}' was missing.")]
    SagaEntityMissing {
        entity_type: &'static str, // e.g., "Corporation", "Business", "Listing"
        entity_id: Uuid,
        step_description: &'static str, // e.g., "Debit Buyer", "Credit Seller"
    },

    #[error("Saga execution failed during step '{step_description}': {reason}")]
    SagaStepFailed {
        step_description: &'static str,
        reason: String, // For errors other than missing entities
    },

    // Add other specific errors as needed, e.g., for spawn_unit
    #[error("Spawn Unit handler failed: {0}")]
    SpawnUnitError(String), // Example for other handlers

    #[error("An internal error occurred: {0}")]
    InternalError(String),
}

// Remove the old struct ActionError and its From implementations

pub struct SimulationService;

impl Simulationable for SimulationService {
    fn calculate_next_state(
        &self,
        next_game_tick: i64,
        msg_act_slice: Vec<(String, QueuedActionPayload)>,
        message_ids: &mut Vec<String>,
        state: &mut GameState,
    ) -> Vec<DomainActionOutcome> {
        let mut outcomes: Vec<DomainActionOutcome> = Vec::with_capacity(msg_act_slice.len());

        let report_failure = |outcomes: &mut Vec<DomainActionOutcome>,
                              user_uuid: Uuid,
                              request_uuid: Uuid,
                              action_string: String,
                              reason: String, // Use the error's Display string
                              tick_processed: i64| {
            failure_outcome()
                .outcomes(outcomes)
                .user_uuid(user_uuid)
                .request_uuid(request_uuid)
                .action(action_string)
                .reason(reason)
                .tick_processed(tick_processed)
                .call();
        };

        for (message_id, action) in msg_act_slice.into_iter() {
            message_ids.push(message_id);
            let action_string = action.details.to_string();
            let user_uuid = action.user_uuid;
            let request_uuid = action.request_uuid;

            let result = match action.details {
                ActionDetails::SpawnUnit => handle_spawn_unit(state, &action, next_game_tick),
                ActionDetails::AcquireListedBusiness { business_uuid } => {
                    handle_acquire_listed_business(state, &action, business_uuid, next_game_tick)
                }
            };

            match result {
                Ok(success_outcome) => {
                    outcomes.push(success_outcome);
                }
                // Match on the specific ActionError enum
                Err(error) => {
                    // Use the error's Display implementation for the reason string
                    report_failure(
                        &mut outcomes,
                        user_uuid,
                        request_uuid,
                        action_string,
                        error.to_string(), // Get reason string from the error itself
                        next_game_tick,
                    );
                }
            }
        }
        outcomes
    }
} // End impl Simulationable

#[builder]
fn failure_outcome(
    outcomes: &mut Vec<DomainActionOutcome>,
    user_uuid: Uuid,
    request_uuid: Uuid,
    action: String,
    reason: String,
    tick_processed: i64,
) {
    // Log the failure - changed to warn level
    tracing::warn!(%user_uuid, %request_uuid, %tick_processed, %action, %reason, "Action failed");

    // Push the failure outcome into the outcomes vector
    outcomes.push(DomainActionOutcome::ActionFailed {
        user_uuid,
        request_uuid,
        reason, // The reason is now the formatted error message
        tick_processed,
    });
}
