use crate::{
    application::action::QueuedActionPayload,
    domain::{
        outcome::DomainActionOutcome,
        simulation::{game_state::GameState, ActionError},
        unit::model::Unit,
    },
};
use bon::builder;
use uuid::Uuid;

#[builder]
pub fn handle_spawn_unit(
    state: &mut GameState,
    action: &QueuedActionPayload,
    next_game_tick: i64,
    req_user_uuid: Uuid,
) -> Result<DomainActionOutcome, ActionError> {
    let unit_uuid = Uuid::now_v7();

    let corporation_uuid = *state
        .corporation_uuid_by_user_uuid
        .get(&req_user_uuid)
        .ok_or(ActionError::RequestingCorporationNotFoundByUser {
            // Use specific error
            user_uuid: req_user_uuid,
        })?;

    let unit = Unit {
        uuid: unit_uuid,
        corporation_uuid,
    };

    state.add_unit(unit);

    Ok(DomainActionOutcome::UnitSpawned {
        req_user_uuid,
        request_uuid: action.request_uuid,
        corporation_uuid,
        unit_uuid,
        tick_effective: next_game_tick,
    })
}
