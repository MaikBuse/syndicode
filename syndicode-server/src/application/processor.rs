use super::{
    economy::list_corporations::ListCorporationsUseCase,
    ports::{processor::GameTickProcessable, queue::ActionQueuer, uow::UnitOfWork},
    warfare::list_units::ListUnitsUseCase,
};
use crate::{
    application::action::QueuedAction,
    domain::{
        corporation::repository::CorporationRepository,
        unit::{model::Unit, repository::UnitRepository},
    },
};
use anyhow::Context;
use std::sync::Arc;
use uuid::Uuid;

pub struct GameTickProcessor<Q, UOW, UNT, CRP>
where
    Q: ActionQueuer,
    UOW: UnitOfWork,
    UNT: UnitRepository,
    CRP: CorporationRepository,
{
    action_queuer: Arc<Q>,
    uow: Arc<UOW>,
    list_units_uc: Arc<ListUnitsUseCase<UNT>>,
    list_corporations_uc: Arc<ListCorporationsUseCase<CRP>>,
}

impl<Q, UOW, UNT, CRP> GameTickProcessor<Q, UOW, UNT, CRP>
where
    Q: ActionQueuer,
    UOW: UnitOfWork,
    UNT: UnitRepository,
    CRP: CorporationRepository,
{
    pub fn new(
        action_queuer: Arc<Q>,
        uow: Arc<UOW>,
        list_units_uc: Arc<ListUnitsUseCase<UNT>>,
        list_corporations_uc: Arc<ListCorporationsUseCase<CRP>>,
    ) -> Self {
        Self {
            action_queuer,
            uow,
            list_units_uc,
            list_corporations_uc,
        }
    }
}

#[tonic::async_trait]
impl<Q, UOW, UNT, CRP> GameTickProcessable for GameTickProcessor<Q, UOW, UNT, CRP>
where
    Q: ActionQueuer,
    UOW: UnitOfWork,
    UNT: UnitRepository,
    CRP: CorporationRepository,
{
    async fn process_next_tick(&self) -> anyhow::Result<usize> {
        // 1. Read Current State & Tick (N) from Repository
        let mut units = self.list_units_uc.execute().await?;
        let mut corporations = self.list_corporations_uc.execute().await?;

        // 2. Pull Actions
        let actions = self.action_queuer.pull_all_available_actions().await?;

        tracing::debug!(num_actions = actions.len(), "Pulled actions.");

        // 3. Calculate State N+1
        'for_action: for (message_id, action) in actions {
            match action {
                QueuedAction::SpawnUnit { req_user_uuid } => {
                    let unit = Unit {
                        uuid: Uuid::now_v7(),
                        user_uuid: req_user_uuid,
                    };

                    units.push(unit);
                }
                QueuedAction::UpdateCorporation { corporation } => {
                    let Some(corporation_to_update) =
                        corporations.iter_mut().find(|c| c.uuid == corporation.uuid)
                    else {
                        tracing::warn!(
                            "Failed to find corporation with uuid '{}'",
                            corporation.uuid
                        );
                        continue 'for_action;
                    };

                    *corporation_to_update = corporation;
                }
            }
        }

        tracing::debug!("Calculated next state.");

        // 4. Write State N+1 Atomically
        let next_game_tick = self
            .uow
            .execute(|ctx| {
                Box::pin(async move {
                    let current_game_tick = ctx.get_current_game_tick().await?;

                    let current_game_tick = usize::try_from(current_game_tick)
                        .map_err(|err| anyhow::format_err!(err))?;

                    let next_game_tick = current_game_tick + 1;

                    ctx.insert_units_in_tick(next_game_tick, units).await?;

                    ctx.delete_units_before_tick(current_game_tick).await?;

                    Ok(next_game_tick)
                })
            })
            .await?;

        tracing::debug!("Atomically wrote next state.");

        // 5. Acknowledge processed actions
        if !actions.is_empty() {
            let action_ids: Vec<&str> = actions.iter().map(|a| a.stream_id.as_str()).collect();
            self.action_queuer
                .acknowledge_actions(&action_ids)
                .await
                .context("Failed to acknowledge actions")?;

            tracing::debug!(
                num_acked = action_ids.len(),
                "Acknowledged processed actions."
            );
        }

        Ok(next_game_tick)
    }
}
