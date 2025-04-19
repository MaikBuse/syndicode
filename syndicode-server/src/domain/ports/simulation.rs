use crate::{
    application::action::QueuedActionPayload,
    domain::{corporation::model::Corporation, outcome::DomainActionOutcome, unit::model::Unit},
};

pub trait Simulationable: Send + Sync {
    /// Calculates the state for the next tick (N+1) based on
    /// the current state (N) and the actions submitted during tick N.
    /// It returns the new state and a vector of specific outcomes generated
    /// by processing the actions.
    /// This MUST be deterministic.
    fn calculate_next_state(
        &self,
        next_game_tick: i64,
        act_msg_slice: Vec<(String, QueuedActionPayload)>,
        messages_ids: &mut Vec<String>,
        units: &mut Vec<Unit>,
        corporations: &mut Vec<Corporation>,
    ) -> Vec<DomainActionOutcome>;
}
