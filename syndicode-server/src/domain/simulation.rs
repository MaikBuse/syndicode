pub mod game_state;
mod handlers;
mod processors;
mod saga;

use super::{outcome::DomainActionOutcome, ports::simulation::Simulationable};
use crate::application::action::{ActionDetails, QueuedAction};
use bon::builder;
use game_state::GameState;
use handlers::{
    acquire_listed_business::handle_acquire_listed_business,
    create_corporation::handle_create_corporation, delete_corporation::handle_delete_corporation,
    spawn_unit::handle_spawn_unit,
};
use processors::business_income::calculate_business_income;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, PartialEq)]
pub enum ActionError {
    #[error("User '{user_uuid}' not associated with any corporation.")]
    RequestingCorporationNotFoundByUser { user_uuid: Uuid },

    #[error("Business '{business_uuid}' not found.")]
    BusinessNotFound { business_uuid: Uuid },

    #[error("Business listing '{listing_uuid}' not found.")]
    BusinessListingNotFound { listing_uuid: Uuid },

    #[error("Unit '{unit_uuid}' not found.")]
    UnitNotFound { unit_uuid: Uuid },

    #[error("Business offer '{offer_uuid}' not found.")]
    BusinessOfferNotFound { offer_uuid: Uuid },

    #[error("Corporation'{corporation_uuid}' not found.")]
    CorporationNotFound { corporation_uuid: Uuid },

    #[error("Corporation'{corporation_uuid}' was not captured.")]
    CorporationNotCaptured { corporation_uuid: Uuid },

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

    #[error("An internal error occurred: {0}")]
    InternalError(String),
}

pub struct SimulationService;

impl Simulationable for SimulationService {
    fn calculate_next_state(
        &self,
        next_game_tick: i64,
        mut queued_actions: Vec<QueuedAction>,
        action_ids: &mut Vec<String>,
        state: &mut GameState,
    ) -> Vec<DomainActionOutcome> {
        let mut outcomes: Vec<DomainActionOutcome> = Vec::with_capacity(queued_actions.len());

        // Sort the actions so that they are executed in the correct order
        queued_actions.sort_by(|a_action, b_action| {
            a_action
                .payload
                .details
                .get_order()
                .cmp(&b_action.payload.details.get_order())
        });

        for queued_action in queued_actions.into_iter() {
            action_ids.push(queued_action.id.clone());
            let action_string = queued_action.payload.details.to_string();
            let req_user_uuid = queued_action.payload.req_user_uuid;
            let request_uuid = queued_action.payload.request_uuid;

            let result = match &queued_action.payload.details {
                ActionDetails::CreateCorporation {
                    user_uuid,
                    corporation_name,
                } => handle_create_corporation()
                    .state(state)
                    .corporation_name(corporation_name.to_owned())
                    .action_payload(&queued_action.payload)
                    .next_game_tick(next_game_tick)
                    .user_uuid(*user_uuid)
                    .req_user_uuid(req_user_uuid)
                    .call(),
                ActionDetails::DeleteCorporation { corporation_uuid } => {
                    handle_delete_corporation()
                        .state(state)
                        .action_payload(&queued_action.payload)
                        .next_game_tick(next_game_tick)
                        .corporation_uuid(*corporation_uuid)
                        .req_user_uuid(req_user_uuid)
                        .call()
                }
                ActionDetails::SpawnUnit => handle_spawn_unit()
                    .state(state)
                    .action_payload(&queued_action.payload)
                    .next_game_tick(next_game_tick)
                    .req_user_uuid(req_user_uuid)
                    .call(),
                ActionDetails::AcquireListedBusiness {
                    business_listing_uuid,
                } => handle_acquire_listed_business()
                    .state(state)
                    .action_payload(&queued_action.payload)
                    .business_listing_uuid(*business_listing_uuid)
                    .next_game_tick(next_game_tick)
                    .req_user_uuid(req_user_uuid)
                    .call(),
            };

            match result {
                Ok(success_outcome) => {
                    outcomes.push(success_outcome);
                }
                // Match on the specific ActionError enum
                Err(error) => {
                    // Use the error's Display implementation for the reason string
                    failure_outcome()
                        .outcomes(&mut outcomes)
                        .req_user_uuid(req_user_uuid)
                        .request_uuid(request_uuid)
                        .action(action_string)
                        .reason(error.to_string())
                        .tick_processed(next_game_tick)
                        .call();
                }
            }
        }

        calculate_business_income(state);

        outcomes
    }
}

#[builder]
fn failure_outcome(
    outcomes: &mut Vec<DomainActionOutcome>,
    req_user_uuid: Uuid,
    request_uuid: Uuid,
    action: String,
    reason: String,
    tick_processed: i64,
) {
    // Log the failure - changed to warn level
    tracing::warn!(%req_user_uuid, %request_uuid, %tick_processed, %action, %reason, "Action failed");

    // Push the failure outcome into the outcomes vector
    outcomes.push(DomainActionOutcome::ActionFailed {
        req_user_uuid,
        request_uuid,
        reason, // The reason is now the formatted error message
        tick_processed,
    });
}
