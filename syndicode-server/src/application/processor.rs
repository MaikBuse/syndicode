use super::{
    economy::{
        list_business_listings::ListBusinessListingUseCase,
        list_business_offers::ListBusinessOffersUseCase, list_businesses::ListBusinessesUseCase,
        list_corporations::ListCorporationsUseCase, list_markets::ListMarketsUseCase,
    },
    ports::{
        game_tick::GameTickRepository,
        init::InitializationRepository,
        outcome::{OutcomeNotifier, OutcomeStoreWriter},
        processor::{GameTickProcessable, ProcessorResult},
        puller::ActionPullable,
        uow::UnitOfWork,
    },
    warfare::list_units::ListUnitsUseCase,
};
use crate::{
    application::ports::processor::ProcessorError,
    domain::{
        economy::{
            business::{model::Business, repository::BusinessRepository},
            business_listing::{model::BusinessListing, repository::BusinessListingRepository},
            business_offer::{model::BusinessOffer, repository::BusinessOfferRepository},
            corporation::{model::Corporation, repository::CorporationRepository},
            market::{model::Market, repository::MarketRepository},
        },
        outcome::DomainActionOutcome,
        ports::simulation::Simulationable,
        simulation::game_state::GameState,
        unit::{model::Unit, repository::UnitRepository},
    },
};
use anyhow::Context;
use bon::Builder;
use std::sync::Arc;
use tokio::sync::OnceCell;

#[derive(Builder)]
pub struct GameTickProcessor<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP, MRK, BSN, BL, BO>
where
    INI: InitializationRepository,
    S: Simulationable,
    P: ActionPullable,
    RSW: OutcomeStoreWriter,
    RN: OutcomeNotifier,
    UOW: UnitOfWork,
    GTR: GameTickRepository,
    UNT: UnitRepository,
    CRP: CorporationRepository,
    MRK: MarketRepository,
    BSN: BusinessRepository,
    BL: BusinessListingRepository,
    BO: BusinessOfferRepository,
{
    init_check_cell: OnceCell<()>, // Stores Ok(()) on successful check
    init_repo: Arc<INI>,
    simulation: Arc<S>,
    action_puller: Arc<P>,
    outcome_store_writer: Arc<RSW>,
    outcome_notifier: Arc<RN>,
    uow: Arc<UOW>,
    game_tick_repo: Arc<GTR>,
    list_units_uc: Arc<ListUnitsUseCase<UNT>>,
    list_corporations_uc: Arc<ListCorporationsUseCase<CRP>>,
    list_markets_uc: Arc<ListMarketsUseCase<MRK>>,
    list_businesses_uc: Arc<ListBusinessesUseCase<BSN>>,
    list_business_listings_uc: Arc<ListBusinessListingUseCase<BL>>,
    list_business_offers_uc: Arc<ListBusinessOffersUseCase<BO>>,
}

impl<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP, MRK, BSN, BL, BO>
    GameTickProcessor<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP, MRK, BSN, BL, BO>
where
    INI: InitializationRepository,
    S: Simulationable,
    P: ActionPullable,
    RSW: OutcomeStoreWriter,
    RN: OutcomeNotifier,
    UOW: UnitOfWork,
    GTR: GameTickRepository,
    UNT: UnitRepository,
    CRP: CorporationRepository,
    MRK: MarketRepository,
    BSN: BusinessRepository,
    BL: BusinessListingRepository,
    BO: BusinessOfferRepository,
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
impl<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP, MRK, BSN, BL, BO> GameTickProcessable
    for GameTickProcessor<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP, MRK, BSN, BL, BO>
where
    INI: InitializationRepository,
    S: Simulationable,
    P: ActionPullable,
    RSW: OutcomeStoreWriter,
    RN: OutcomeNotifier,
    UOW: UnitOfWork,
    GTR: GameTickRepository,
    UNT: UnitRepository,
    CRP: CorporationRepository,
    MRK: MarketRepository,
    BSN: BusinessRepository,
    BL: BusinessListingRepository,
    BO: BusinessOfferRepository,
{
    async fn process_next_tick(&self) -> ProcessorResult<i64> {
        // 0. CHECK DATABASE INITIALIZATION
        self.init_check_cell
            .get_or_try_init(|| self.perform_db_initialization_check())
            .await?; // Propagates ProcessorError::NotInitialized or CheckFailed if check fails

        // 1. Read Current State & Tick (N) from Repositories
        let current_game_tick = self.game_tick_repo.get_current_game_tick().await?;
        let next_game_tick = current_game_tick + 1;

        let units_vec = self.list_units_uc.execute(current_game_tick).await?;
        let corporations_vec = self.list_corporations_uc.execute(current_game_tick).await?;
        let markets_vec = self.list_markets_uc.execute(current_game_tick).await?;
        let businesses_vec = self.list_businesses_uc.execute(current_game_tick).await?;
        let business_listings_vec = self
            .list_business_listings_uc
            .execute(current_game_tick)
            .await?;
        let business_offers_vec = self
            .list_business_offers_uc
            .execute(current_game_tick)
            .await?;

        let mut game_state = GameState::build(
            units_vec,
            corporations_vec,
            markets_vec,
            businesses_vec,
            business_listings_vec,
            business_offers_vec,
        );

        // 2. Pull Actions
        let queued_actions = self.action_puller.pull_all_available_actions().await?;
        let act_msg_count = queued_actions.len();

        tracing::debug!(num_actions = queued_actions.len(), "Pulled actions.");

        // 3. Calculate State N+1
        let mut action_ids: Vec<String> = Vec::with_capacity(queued_actions.len());

        let action_outcomes = self.simulation.calculate_next_state(
            next_game_tick,
            queued_actions,
            &mut action_ids,
            &mut game_state,
        );

        tracing::debug!("Calculated next state.");

        // 4. Write State N+1 Atomically
        let units: Vec<Unit> = game_state.units_map.into_values().collect();
        let corporations: Vec<Corporation> = game_state.corporations_map.into_values().collect();
        let markets: Vec<Market> = game_state.markets_map.into_values().collect();
        let businesses: Vec<Business> = game_state.businesses_map.into_values().collect();
        let business_listings: Vec<BusinessListing> =
            game_state.business_listings_map.into_values().collect();
        let business_offers: Vec<BusinessOffer> =
            game_state.business_offers_map.into_values().collect();

        let action_outcomes = self
            .uow
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

                    // Markets
                    ctx.insert_markets_in_tick(next_game_tick, markets).await?;
                    ctx.delete_markets_before_tick(current_game_tick).await?;

                    // Businesses
                    ctx.insert_businesses_in_tick(next_game_tick, businesses)
                        .await?;
                    ctx.delete_businesses_before_tick(current_game_tick).await?;

                    // Business Listings
                    ctx.insert_business_listings_in_tick(next_game_tick, business_listings)
                        .await?;
                    ctx.delete_business_listings_before_tick(current_game_tick)
                        .await?;

                    // Business Offers
                    ctx.insert_business_offers_in_tick(next_game_tick, business_offers)
                        .await?;
                    ctx.delete_business_offers_before_tick(current_game_tick)
                        .await?;

                    // Update game tick state
                    ctx.update_current_game_tick(next_game_tick).await?;

                    Ok(action_outcomes)
                })
            })
            .await?;

        tracing::debug!("Atomically wrote next state.");

        // 5. Handle domain events

        // 6. Acknowledge processed actions
        if act_msg_count != 0 {
            let message_count = action_ids.len();

            self.action_puller
                .acknowledge_actions(action_ids)
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
                let request_uuid = outcome.get_request_uuid();
                let user_uuid = outcome.get_req_user_uuid();

                // a. Store the full outcome/payload
                // Serialize the specific data needed for the final response
                let result_payload = self.serialize_outcome_for_delivery(&outcome)?; // Helper needed

                self.outcome_store_writer
                    .store_outcome(request_uuid, &result_payload)
                    .await?; // Handle store error

                // b. Publish notification
                self.outcome_notifier
                    .notify_outcome_ready(user_uuid, request_uuid)
                    .await?; // Handle notify error
            }
        }

        // 7. Send notification that game state has advanced
        self.outcome_notifier
            .notify_game_tick_advanced(next_game_tick)
            .await?;

        Ok(next_game_tick)
    }
}
