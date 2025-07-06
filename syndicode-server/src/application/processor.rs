use super::{
    economy::{
        list_building_ownerships::ListBuildingOwnershipsUseCase,
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
    application::ports::{init::FlagKey, processor::ProcessorError},
    domain::{
        economy::{
            building_ownership::{
                model::BuildingOwnership, repository::BuildingOwnershipRepository,
            },
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
use tokio::sync::Mutex;

#[derive(Builder)]
pub struct GameTickProcessor<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP, MRK, BSN, BL, BO, BLO>
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
    BLO: BuildingOwnershipRepository,
{
    init_repo: Arc<INI>,
    state: Arc<Mutex<Option<GameState>>>,
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
    list_building_ownerships: Arc<ListBuildingOwnershipsUseCase<BLO>>,
}

impl<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP, MRK, BSN, BL, BO, BLO>
    GameTickProcessor<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP, MRK, BSN, BL, BO, BLO>
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
    BLO: BuildingOwnershipRepository,
{
    // Helper to serialize outcomes
    fn serialize_outcome_for_delivery(
        &self,
        outcome: &DomainActionOutcome,
    ) -> Result<Vec<u8>, anyhow::Error> {
        rmp_serde::to_vec(outcome).context("Failed to serialize outcome for delivery")
    }

    /// This function performs the initial, one-time load of the game state from the database.
    /// It's called only when the processor's internal state is empty.
    async fn initialize_state(&self) -> ProcessorResult<GameState> {
        // First, confirm the database itself has been initialized.
        let is_db_initialized = self.init_repo.is_flag_set(FlagKey::Database).await?;
        if !is_db_initialized {
            tracing::warn!("Processor waiting: Database initialization flag is not yet set.");
            return Err(ProcessorError::NotInitialized);
        }
        tracing::info!("Database initialization confirmed. Loading full game state...");

        // Load all state components from the database
        let current_game_tick = self.game_tick_repo.get_current_game_tick().await?;

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
        let building_ownerships_vec = self
            .list_building_ownerships
            .execute(current_game_tick)
            .await?;

        let game_state = GameState::build()
            .last_processed_tick(current_game_tick)
            .units_vec(units_vec)
            .corporations_vec(corporations_vec)
            .markets_vec(markets_vec)
            .businesses_vec(businesses_vec)
            .business_listings_vec(business_listings_vec)
            .business_offers_vec(business_offers_vec)
            .building_ownerships_vec(building_ownerships_vec)
            .call();

        tracing::info!(
            tick = current_game_tick,
            "Successfully loaded initial game state into memory."
        );

        Ok(game_state)
    }
}

#[tonic::async_trait]
impl<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP, MRK, BSN, BL, BO, BLO> GameTickProcessable
    for GameTickProcessor<INI, S, P, RSW, RN, UOW, GTR, UNT, CRP, MRK, BSN, BL, BO, BLO>
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
    BLO: BuildingOwnershipRepository,
{
    async fn process_next_tick(&self) -> ProcessorResult<i64> {
        // Acquire a lock on the state. This lock is held for the entire tick processing.
        let mut state_guard = self.state.lock().await;

        // 1. CHECK & INITIALIZE STATE
        // If state is `None`, this is the first run. We must load everything.
        if state_guard.is_none() {
            let initial_state = self.initialize_state().await?;
            *state_guard = Some(initial_state);

            // IMPORTANT: Return `NotInitialized` to signal that we only performed setup.
            // The LeaderLoopManager will wait for the next scheduled tick time to call us again.
            // This decouples the potentially slow initial load from the timed tick processing.
            return Err(ProcessorError::NotInitialized);
        }

        // We are guaranteed to have state now. Take ownership to work with it.
        // We will put it back (or an updated version) before the lock is released.
        let mut game_state = state_guard.take().unwrap();
        let current_game_tick = game_state.last_processed_tick;
        let next_game_tick = current_game_tick + 1;

        // 2. Pull Actions (This happens every tick)
        let queued_actions = self.action_puller.pull_all_available_actions().await?;
        let act_msg_count = queued_actions.len();
        tracing::debug!(
            num_actions = act_msg_count,
            "Pulled actions for tick {}.",
            next_game_tick
        );

        // 3. Calculate State N+1 (using the in-memory game_state)
        let mut action_ids: Vec<String> = Vec::with_capacity(act_msg_count);
        let action_outcomes = self.simulation.calculate_next_state(
            next_game_tick,
            queued_actions,
            &mut action_ids,
            &mut game_state,
        );
        tracing::debug!("Calculated next state in memory.");

        // 4. Write State N+1 Atomically
        // Convert the owned HashMaps into owned Vecs.
        // This is very cheap because it just moves the values, no deep clones.
        let units: Vec<Unit> = std::mem::take(&mut game_state.units_map)
            .into_values()
            .collect();
        let corporations: Vec<Corporation> = std::mem::take(&mut game_state.corporations_map)
            .into_values()
            .collect();
        let markets: Vec<Market> = std::mem::take(&mut game_state.markets_map)
            .into_values()
            .collect();
        let businesses: Vec<Business> = std::mem::take(&mut game_state.businesses_map)
            .into_values()
            .collect();
        let business_listings: Vec<BusinessListing> =
            std::mem::take(&mut game_state.business_listings_map)
                .into_values()
                .collect();
        let business_offers: Vec<BusinessOffer> =
            std::mem::take(&mut game_state.business_offers_map)
                .into_values()
                .collect();
        let building_ownerships: Vec<BuildingOwnership> =
            std::mem::take(&mut game_state.building_ownerships_map)
                .into_values()
                .collect();

        let (
            units,
            corporations,
            markets,
            businesses,
            business_listings,
            business_offers,
            building_ownerships,
        ) = self
            .uow
            .execute(move |ctx| {
                Box::pin(async move {
                    // Units
                    ctx.insert_units_in_tick(next_game_tick, &units).await?;
                    ctx.delete_units_before_tick(current_game_tick).await?;

                    // Corporations
                    ctx.insert_corporations_in_tick(next_game_tick, &corporations)
                        .await?;
                    ctx.delete_corporations_before_tick(current_game_tick)
                        .await?;

                    // Markets
                    ctx.insert_markets_in_tick(next_game_tick, &markets).await?;
                    ctx.delete_markets_before_tick(current_game_tick).await?;

                    // Businesses
                    ctx.insert_businesses_in_tick(next_game_tick, &businesses)
                        .await?;
                    ctx.delete_businesses_before_tick(current_game_tick).await?;

                    // Business Listings
                    ctx.insert_business_listings_in_tick(next_game_tick, &business_listings)
                        .await?;
                    ctx.delete_business_listings_before_tick(current_game_tick)
                        .await?;

                    // Business Offers
                    ctx.insert_business_offers_in_tick(next_game_tick, &business_offers)
                        .await?;
                    ctx.delete_business_offers_before_tick(current_game_tick)
                        .await?;

                    // Building Ownerships
                    ctx.insert_building_ownerships_in_tick(next_game_tick, &building_ownerships)
                        .await?;
                    ctx.delete_building_ownerships_before_tick(current_game_tick)
                        .await?;

                    // Game Tick Update
                    ctx.update_current_game_tick(next_game_tick).await?;

                    Ok((
                        units,
                        corporations,
                        markets,
                        businesses,
                        business_listings,
                        business_offers,
                        building_ownerships,
                    ))
                })
            })
            .await?;

        // --- REBUILD THE STATE ---
        // Whether the UOW succeeded or failed, we must restore the full game_state
        // object before releasing the lock, so it's ready for the next attempt.
        // We do this by converting the vectors back into HashMaps.
        game_state.corporations_map = corporations.into_iter().map(|c| (c.uuid, c)).collect();
        game_state.units_map = units.into_iter().map(|u| (u.uuid, u)).collect();
        game_state.markets_map = markets.into_iter().map(|m| (m.uuid, m)).collect();
        game_state.businesses_map = businesses.into_iter().map(|b| (b.uuid, b)).collect();
        game_state.business_listings_map = business_listings
            .into_iter()
            .map(|bl| (bl.uuid, bl))
            .collect();
        game_state.business_offers_map = business_offers
            .into_iter()
            .map(|bo| (bo.uuid, bo))
            .collect();
        game_state.building_ownerships_map = building_ownerships
            .into_iter()
            .map(|buw| (buw.building_uuid, buw))
            .collect();
        game_state.last_processed_tick = next_game_tick;

        *state_guard = Some(game_state);

        tracing::debug!("Atomically wrote state for tick {}.", next_game_tick);

        // 5. Acknowledge and Notify
        if act_msg_count != 0 {
            self.action_puller.acknowledge_actions(action_ids).await?;
            tracing::debug!(num_acked = act_msg_count, "Acknowledged processed actions.");
        }

        if !action_outcomes.is_empty() {
            for outcome in action_outcomes {
                let request_uuid = outcome.get_request_uuid();
                let user_uuid = outcome.get_req_user_uuid();
                let result_payload = self.serialize_outcome_for_delivery(&outcome)?;
                self.outcome_store_writer
                    .store_outcome(request_uuid, &result_payload)
                    .await?;
                self.outcome_notifier
                    .notify_outcome_ready(user_uuid, request_uuid)
                    .await?;
            }
        }

        self.outcome_notifier
            .notify_game_tick_advanced(next_game_tick)
            .await?;

        Ok(next_game_tick)
    }
}
