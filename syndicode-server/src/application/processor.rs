use super::{
    economy::list_corporations::ListCorporationsUseCase,
    ports::{
        game_tick::GameTickRepository,
        init::InitializationRepository,
        processor::{GameTickProcessable, ProcessorResult},
        puller::ActionPullable,
        results::{ResultNotifier, ResultStoreWriter},
        uow::UnitOfWork,
    },
    warfare::list_units::ListUnitsUseCase,
};
use crate::{
    application::ports::processor::ProcessorError,
    domain::{
        economy::corporation::repository::CorporationRepository, outcome::DomainActionOutcome,
        ports::simulation::Simulationable, unit::repository::UnitRepository,
    },
};
use anyhow::Context;
use bon::Builder;
use std::sync::Arc;
use tokio::sync::OnceCell;

#[derive(Builder)]
pub struct GameTickProcessor<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP>
where
    INI: InitializationRepository,
    S: Simulationable,
    P: ActionPullable,
    RSW: ResultStoreWriter,
    RN: ResultNotifier,
    UOW: UnitOfWork,
    GTR: GameTickRepository,
    UNT: UnitRepository,
    CRP: CorporationRepository,
{
    init_check_cell: OnceCell<()>, // Stores Ok(()) on successful check
    init_repo: Arc<INI>,
    simulation: Arc<S>,
    action_puller: Arc<P>,
    result_store_writer: Arc<RSW>,
    result_notifier: Arc<RN>,
    uow: Arc<UOW>,
    game_tick_repo: Arc<GTR>,
    list_units_uc: Arc<ListUnitsUseCase<UNT>>,
    list_corporations_uc: Arc<ListCorporationsUseCase<CRP>>,
}

impl<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP>
    GameTickProcessor<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP>
where
    INI: InitializationRepository,
    S: Simulationable,
    P: ActionPullable,
    RSW: ResultStoreWriter,
    RN: ResultNotifier,
    UOW: UnitOfWork,
    GTR: GameTickRepository,
    UNT: UnitRepository,
    CRP: CorporationRepository,
{
    // Helper to serialize the outcome into bytes for storage
    fn serialize_outcome_for_delivery(
        &self,
        outcome: &DomainActionOutcome,
    ) -> Result<Vec<u8>, anyhow::Error> {
        rmp_serde::to_vec(outcome).context("Failed to serialize outcome for delivery")
    }

    async fn perform_db_initialization_check(&self) -> ProcessorResult<()> {
        tracing::debug!("Performing one-time database initialization check...");

        let is_db_ini = self.init_repo.is_database_initialized().await?;

        match is_db_ini {
            true => {
                tracing::info!("Database initialization confirmed by processor.");
                Ok(()) // Success, store Ok(()) in OnceCell
            }
            false => {
                tracing::warn!("Database initialization flag not yet set.");
                Err(ProcessorError::NotInitialized) // Signal not ready
            }
        }
    }
}

#[tonic::async_trait]
impl<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP> GameTickProcessable
    for GameTickProcessor<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP>
where
    INI: InitializationRepository,
    S: Simulationable,
    P: ActionPullable,
    RSW: ResultStoreWriter,
    RN: ResultNotifier,
    UOW: UnitOfWork,
    GTR: GameTickRepository,
    UNT: UnitRepository,
    CRP: CorporationRepository,
{
    async fn process_next_tick(&self) -> ProcessorResult<i64> {
        // 0. CHECK DATABASE INITIALIZATION
        self.init_check_cell
            .get_or_try_init(|| self.perform_db_initialization_check())
            .await?; // Propagates ProcessorError::NotInitialized or CheckFailed if check fails

        // 1. Read Current State & Tick (N) from Repositories
        let current_game_tick = self.game_tick_repo.get_current_game_tick().await?;
        let next_game_tick = current_game_tick + 1;

        let mut units = self.list_units_uc.execute().await?;
        let mut corporations = self.list_corporations_uc.execute().await?;

        // 2. Pull Actions
        let act_msg_slice = self.action_puller.pull_all_available_actions().await?;
        let act_msg_count = act_msg_slice.len();

        tracing::debug!(num_actions = act_msg_slice.len(), "Pulled actions.");

        // 3. Calculate State N+1
        let mut messages_ids: Vec<String> = Vec::with_capacity(act_msg_slice.len());

        let action_outcomes = self.simulation.calculate_next_state(
            next_game_tick,
            act_msg_slice,
            &mut messages_ids,
            &mut units,
            &mut corporations,
        );

        tracing::debug!("Calculated next state.");

        // 4. Write State N+1 Atomically
        self.uow
            .execute(move |ctx| {
                Box::pin(async move {
                    // Units
                    ctx.insert_units_in_tick(next_game_tick, units).await?;
                    ctx.delete_units_before_tick(current_game_tick).await?;

                    // Corporations
                    ctx.insert_corporations_in_tick(next_game_tick, corporations)
                        .await?;
                    ctx.delete_corporations_before_tick(current_game_tick)
                        .await?;

                    // Update game tick state
                    ctx.update_current_game_tick(next_game_tick).await?;

                    Ok(())
                })
            })
            .await?;

        tracing::debug!("Atomically wrote next state.");

        // 5. Acknowledge processed actions
        if act_msg_count != 0 {
            let message_count = messages_ids.len();

            self.action_puller
                .acknowledge_actions(messages_ids)
                .await
                .context("Failed to acknowledge actions")?;

            tracing::debug!(num_acked = message_count, "Acknowledged processed actions.");
        }

        // 6. Store Results and Notify
        if !action_outcomes.is_empty() {
            tracing::debug!(
                tick = next_game_tick,
                count = action_outcomes.len(),
                "Storing results and notifying."
            );

            // Need to map outcome back to its request_uuid
            // This requires the SimulationService/Handlers to pass it through
            for outcome in action_outcomes {
                // Assuming outcome includes request_uuid and user_uuid
                let (request_uuid, user_uuid) = match &outcome {
                    DomainActionOutcome::UnitSpawned {
                        request_uuid,
                        user_uuid,
                        ..
                    } => (*request_uuid, *user_uuid),
                    DomainActionOutcome::ActionFailed {
                        request_uuid,
                        user_uuid,
                        ..
                    } => (*request_uuid, *user_uuid),
                };

                // a. Store the full outcome/payload
                // Serialize the specific data needed for the final response
                let result_payload = self.serialize_outcome_for_delivery(&outcome)?; // Helper needed

                self.result_store_writer
                    .store_result(request_uuid, &result_payload)
                    .await?; // Handle store error

                // b. Publish notification
                self.result_notifier
                    .notify_result_ready(user_uuid, request_uuid)
                    .await?; // Handle notify error
            }
        }

        Ok(next_game_tick)
    }
}
