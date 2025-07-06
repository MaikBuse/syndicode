use bon::builder;
use rand::{distr::Alphanumeric, Rng};
use uuid::Uuid;

use crate::{
    application::action::QueuedActionPayload,
    domain::{
        economy::corporation::model::{name::CorporationName, Corporation},
        outcome::DomainActionOutcome,
        simulation::{game_state::GameState, ActionError},
    },
};

#[builder]
pub fn handle_create_corporation(
    state: &mut GameState,
    action_payload: &QueuedActionPayload,
    next_game_tick: i64,
    user_uuid: Uuid,
    mut corporation_name: CorporationName,
    req_user_uuid: Uuid,
) -> Result<DomainActionOutcome, ActionError> {
    // If the corporation name is already taken, generate a random suffix for it
    while state.corporation_names.contains(corporation_name.as_str()) {
        let suffix: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(3)
            .map(char::from)
            .collect();

        corporation_name = CorporationName::unchecked(format!("{corporation_name}-{suffix}"));
    }

    let corporation_name_string = corporation_name.to_string();

    let corporation = Corporation::new(user_uuid, corporation_name);

    let outcome = DomainActionOutcome::CorporationCreated {
        corporation_uuid: corporation.uuid,
        corporation_name: corporation.name.to_string(),
        corporation_balance: corporation.cash_balance,
        request_uuid: action_payload.request_uuid,
        tick_effective: next_game_tick,
        req_user_uuid,
        user_uuid,
    };

    state.corporation_names.insert(corporation_name_string);
    state.corporations_map.insert(corporation.uuid, corporation);

    Ok(outcome)
}
