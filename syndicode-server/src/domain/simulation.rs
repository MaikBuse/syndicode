use super::{
    economy::corporation::model::Corporation, outcome::DomainActionOutcome,
    ports::simulation::Simulationable, unit::model::Unit,
};
use crate::application::action::{ActionDetails, QueuedActionPayload};
use uuid::Uuid;

pub struct SimulationService;

impl Simulationable for SimulationService {
    fn calculate_next_state(
        &self,
        next_game_tick: i64,
        act_msg_slice: Vec<(String, QueuedActionPayload)>,
        messages_ids: &mut Vec<String>,
        units: &mut Vec<Unit>,
        corporations: &mut Vec<Corporation>,
    ) -> Vec<DomainActionOutcome> {
        let mut outcomes: Vec<DomainActionOutcome> = Vec::with_capacity(act_msg_slice.len());

        'for_action: for (message_id, action) in act_msg_slice.into_iter() {
            messages_ids.push(message_id);

            match &action.details {
                ActionDetails::SpawnUnit { req_user_uuid } => {
                    let unit_uuid = Uuid::now_v7();

                    let unit = Unit {
                        uuid: unit_uuid,
                        user_uuid: *req_user_uuid,
                    };

                    units.push(unit);

                    outcomes.push(DomainActionOutcome::UnitSpawned {
                        user_uuid: *req_user_uuid,
                        request_uuid: action.request_uuid,
                        unit_uuid,
                        tick_effective: next_game_tick,
                    });
                }
                ActionDetails::UpdateCorporation { corporation } => {
                    let Some(corporation_to_update) =
                        corporations.iter_mut().find(|c| c.uuid == corporation.uuid)
                    else {
                        tracing::warn!(
                            "Failed to find corporation with uuid '{}'",
                            corporation.uuid
                        );
                        continue 'for_action;
                    };

                    *corporation_to_update = corporation.clone();
                }
            }
        }

        outcomes
    }
}
