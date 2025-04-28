use crate::{
    application::action::QueuedActionPayload,
    domain::{
        outcome::DomainActionOutcome,
        simulation::{game_state::GameState, ActionError},
        unit::model::Unit,
    },
};
use uuid::Uuid;

pub fn handle_spawn_unit(
    state: &mut GameState,
    action: &QueuedActionPayload,
    next_game_tick: i64,
) -> Result<DomainActionOutcome, ActionError> {
    let unit_uuid = Uuid::now_v7();
    let unit = Unit {
        uuid: unit_uuid,
        user_uuid: action.user_uuid,
    };
    state.add_unit(unit);
    Ok(DomainActionOutcome::UnitSpawned {
        user_uuid: action.user_uuid,
        request_uuid: action.request_uuid,
        unit_uuid,
        tick_effective: next_game_tick,
    })
}
