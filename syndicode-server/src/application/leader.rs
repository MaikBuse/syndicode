use crate::application::ports::leader::{LeaderElectionError, LeaderElector}; // Assume these are defined
use crate::application::ports::processor::{GameTickProcessable, ProcessorError};
use bon::Builder;
use std::{sync::Arc, time::Duration};
use tokio::sync::OnceCell;
use tokio::time::{self, Instant};

use super::init::InitializationOrchestrator;
use super::ports::crypto::PasswordHandler;
use super::ports::downloader::BackupDownloader;
use super::ports::init::InitializationRepository;
use super::ports::migration::MigrationRunner;
use super::ports::restorer::DatabaseRestorer;
use super::ports::uow::UnitOfWork;

/// Manages the leader election loop and triggers the game tick processor when leader.
#[derive(Builder)]
pub struct LeaderLoopManager<L, G, UOW, INI, RES, DOW, P, M>
where
    L: LeaderElector,
    G: GameTickProcessable,
    UOW: UnitOfWork,
    INI: InitializationRepository,
    RES: DatabaseRestorer,
    DOW: BackupDownloader,
    P: PasswordHandler,
    M: MigrationRunner,
{
    leader_elector: Arc<L>,
    game_tick_processor: Arc<G>,
    instance_id: String,
    leader_lock_refresh_interval: Duration,
    non_leader_acquisition_retry_interval: Duration,
    game_tick_interval: Duration,
    initialization_orchestrator: Arc<InitializationOrchestrator<UOW, INI, RES, DOW, P, M>>,
    #[builder(default = OnceCell::new())]
    initialization_done: OnceCell<()>,
}

impl<L, G, UOW, INI, RES, DOW, P, M> LeaderLoopManager<L, G, UOW, INI, RES, DOW, P, M>
where
    L: LeaderElector + Send + Sync + 'static,
    G: GameTickProcessable + Send + Sync + 'static,
    UOW: UnitOfWork + Send + Sync + 'static,
    INI: InitializationRepository + Send + Sync + 'static,
    RES: DatabaseRestorer + Send + Sync + 'static,
    DOW: BackupDownloader + Send + Sync + 'static,
    P: PasswordHandler + Send + Sync + 'static,
    M: MigrationRunner + Send + Sync + 'static,
{
    /// Runs the leader election loop indefinitely.
    ///
    /// This function is the main entry point and acts as a state machine, delegating
    /// to `handle_leader_state` or `handle_non_leader_state` based on whether this
    /// instance is currently the leader.
    pub async fn run(self) {
        tracing::info!("Starting leader loop election...");

        let mut is_leader = false;
        let mut next_tick_time: Option<Instant> = None;

        loop {
            if is_leader {
                self.handle_leader_state(&mut is_leader, &mut next_tick_time)
                    .await;
            } else {
                self.handle_non_leader_state(&mut is_leader, &mut next_tick_time)
                    .await;
            }
        }
    }

    // --- State Handlers ---

    /// Manages the logic for a single cycle when the instance believes it is the leader.
    ///
    /// It first refreshes the lock. If successful, it processes game ticks.
    /// If the refresh fails or a critical error occurs during processing, it relinquishes leadership.
    async fn handle_leader_state(
        &self,
        is_leader: &mut bool,
        next_tick_time: &mut Option<Instant>,
    ) {
        // First, run the one-time initialization. If it fails, relinquish leadership.
        if self.run_one_time_initialization_if_needed().await.is_err() {
            tracing::warn!("Relinquishing leadership due to initialization failure.");
            self.handle_critical_processor_error(is_leader, next_tick_time)
                .await;
            return;
        }

        match self.leader_elector.refresh().await {
            Ok(()) => {
                tracing::trace!("Leader lock refreshed successfully.");
                // Still leader, so we drive the tick processing and sleeping logic.
                self.drive_tick_processing_cycle(is_leader, next_tick_time)
                    .await;
            }
            Err(LeaderElectionError::NotHoldingLock { key, instance_id }) => {
                tracing::info!(key=%key, current_instance=%self.instance_id , owner_instance=%instance_id, "Lost leadership or lock expired (refresh check failed).");
                *is_leader = false;
                *next_tick_time = None;
                // No sleep here; loop immediately to try re-acquiring leadership.
            }
            Err(e) => {
                // This indicates an error communicating with the lock provider (e.g., network issue).
                self.handle_refresh_error(is_leader, next_tick_time, e)
                    .await;
            }
        }
    }

    /// Manages the logic for a single cycle when the instance is not the leader.
    ///
    /// It attempts to acquire the leader lock and sleeps on failure before the next attempt.
    async fn handle_non_leader_state(
        &self,
        is_leader: &mut bool,
        next_tick_time: &mut Option<Instant>,
    ) {
        tracing::debug!("Not leader. Attempting to acquire lock...");
        match self.leader_elector.try_acquire().await {
            Ok(true) => {
                tracing::info!(instance_id = %self.instance_id, "Successfully acquired leadership!");
                *is_leader = true;
                // Ensure timer is re-initialized on the first leader cycle.
                *next_tick_time = None;
            }
            Ok(false) => {
                // Failed to acquire, someone else is leader. Wait before retrying.
                tracing::debug!(
                    "Failed to acquire lock (already held or unavailable). Retrying after interval."
                );
                time::sleep(self.non_leader_acquisition_retry_interval).await;
            }
            Err(e) => {
                // Error during acquisition attempt. Wait before retrying.
                tracing::error!(
                    error = %e,
                    "Error trying to acquire leader lock. Retrying after interval."
                );
                time::sleep(self.non_leader_acquisition_retry_interval).await;
            }
        }
    }

    async fn run_one_time_initialization_if_needed(&self) -> Result<(), ()> {
        let init_result = self
            .initialization_done
            .get_or_try_init(|| async {
                tracing::info!(
                    "Leader has been elected. Running one-time database initialization..."
                );
                self.initialization_orchestrator.run().await.map_err(|e| {
                    tracing::error!(error = %e, "One-time initialization failed!");
                    e
                })
            })
            .await;

        match init_result {
            Ok(_) => {
                tracing::info!("Initialization is complete. Proceeding with leader duties.");
                Ok(())
            }
            Err(_) => {
                // The error was already logged inside the closure.
                // We return an error to signal that the leader state should terminate.
                Err(())
            }
        }
    }

    // --- Leader-Specific Logic ---

    /// Orchestrates a single cycle of the leader's duties: processing due ticks and then sleeping.
    async fn drive_tick_processing_cycle(
        &self,
        is_leader: &mut bool,
        next_tick_time: &mut Option<Instant>,
    ) {
        // This function will process a single tick if one is due.
        // It returns `true` if a critical, unrecoverable processing error occurred.
        let had_critical_error = self.process_tick_if_due(next_tick_time).await;

        if had_critical_error {
            self.handle_critical_processor_error(is_leader, next_tick_time)
                .await;
        } else if *is_leader {
            // If we are still the leader, calculate the appropriate sleep time
            // until the next event (either a game tick or a lock refresh).
            self.sleep_until_next_event(*next_tick_time).await;
        }
    }

    /// Processes a single game tick if its scheduled time has passed.
    ///
    /// This function does **not** loop to "catch up". Instead, it implements specific logic
    /// for scheduling the next tick based on the processing time of the current one:
    /// - If processing is fast (duration < interval), it maintains a fixed-rate cadence.
    /// - If processing is slow (duration > interval), it schedules the next tick relative to
    ///   the **completion time** of the slow tick. This introduces a mandatory pause to prevent
    ///   the system from being overwhelmed by a runaway catch-up loop.
    ///
    /// Returns `true` if a critical processing error occurs that requires relinquishing
    /// leadership. Returns `false` otherwise.
    async fn process_tick_if_due(&self, next_tick_time: &mut Option<Instant>) -> bool {
        // Initialize the tick timer on the very first run after becoming leader.
        if next_tick_time.is_none() {
            let first_tick_target = Instant::now() + self.game_tick_interval;
            *next_tick_time = Some(first_tick_target);
            tracing::info!(
                "Initialized tick timer. First tick target: {:?}",
                first_tick_target
            );
        }

        // We can unwrap here; it was just set if None.
        let current_tick_target = next_tick_time.unwrap();
        let now = Instant::now();

        // Check if it's time for the next scheduled tick. If not, we're done for this cycle.
        if now < current_tick_target {
            return false; // Not yet time for a tick, not an error.
        }

        let tick_start_offset = now.saturating_duration_since(current_tick_target);
        tracing::debug!(
            lag_ms = tick_start_offset.as_millis(),
            "Starting tick processing for target time: {:?}",
            current_tick_target
        );

        let processing_start_instant = Instant::now();
        match self.game_tick_processor.process_next_tick().await {
            Ok(processed_tick) => {
                let duration = processing_start_instant.elapsed();

                if duration > self.game_tick_interval {
                    // SLOW TICK PATH: The tick took longer than the interval.
                    // To prevent a runaway loop of immediate catch-up ticks, we schedule the
                    // next tick relative to when *this* one finished. This ensures a full
                    // `game_tick_interval` of "cool-down" before the next attempt.
                    // This intentionally introduces drift to maintain system stability under load.
                    *next_tick_time = Some(Instant::now() + self.game_tick_interval);
                    tracing::warn!(
                        duration_ms = duration.as_millis(),
                        target_ms = self.game_tick_interval.as_millis(),
                        "Tick processing duration exceeded target interval. Next tick is scheduled relative to completion to prevent runaway."
                    );
                } else {
                    // FAST TICK PATH: The tick finished within the interval.
                    // We schedule the next tick relative to the target time of the *current*
                    // tick. This maintains a stable, fixed-rate cadence and allows the
                    // system to correct for minor processing delays without long-term drift.
                    *next_tick_time = Some(current_tick_target + self.game_tick_interval);
                }

                tracing::info!(
                    tick = processed_tick,
                    duration_ms = duration.as_millis(),
                    target_interval_ms = self.game_tick_interval.as_millis(),
                    lag_ms = tick_start_offset.as_millis(),
                    "Successfully processed game tick."
                );

                false // Not a critical error.
            }
            Err(ProcessorError::NotInitialized) => {
                tracing::warn!("Tick processing skipped: Database not initialized yet. Will retry on next cycle.");
                // We don't advance the tick timer. The next loop cycle will retry processing
                // for the same `current_tick_target` after a sleep.
                false // Not a critical error.
            }
            Err(err) => {
                // Any other processor error is treated as critical.
                tracing::error!(
                    "Game tick processing failed (ProcessorError: {}). Relinquishing leadership.",
                    err
                );
                true // Critical error.
            }
        }
    }

    /// Calculates the appropriate amount of time to sleep and then awaits that duration.
    ///
    /// The sleep duration is the *minimum* of the time until the next scheduled game tick
    /// and the time until the next required lock refresh.
    async fn sleep_until_next_event(&self, next_tick_time: Option<Instant>) {
        let now = Instant::now();
        // `next_tick_time` should be Some, but we default to `now` for safety.
        let next_tick_due_at = next_tick_time.unwrap_or(now);

        // `saturating_duration_since` handles cases where the tick is already past due (returns Duration::ZERO).
        let time_until_next_tick = next_tick_due_at.saturating_duration_since(now);

        // Check slightly before the refresh interval expires to be safe.
        let time_until_refresh_needed = self.leader_lock_refresh_interval.mul_f32(0.9);

        // We must wake up for whichever event is sooner.
        let sleep_duration = time_until_next_tick.min(time_until_refresh_needed);

        if sleep_duration > Duration::ZERO {
            tracing::trace!(
                "Sleeping for {:?} (until next tick: {:?}, until refresh: {:?})",
                sleep_duration,
                time_until_next_tick,
                time_until_refresh_needed
            );
            time::sleep(sleep_duration).await;
        } else {
            // If sleep duration is zero (e.g., we are behind schedule), yield to allow
            // other async tasks to run and prevent hogging the CPU.
            tracing::trace!("Calculated sleep duration is zero or negative. Yielding.");
            tokio::task::yield_now().await;
        }
    }

    // --- Error Handling Helpers ---

    /// Handles a critical error during game tick processing by relinquishing leadership.
    async fn handle_critical_processor_error(
        &self,
        is_leader: &mut bool,
        next_tick_time: &mut Option<Instant>,
    ) {
        if let Err(release_err) = self.leader_elector.release().await {
            tracing::error!(
                error = %release_err,
                "Failed to release leader lock after critical processing error."
            );
        }
        *is_leader = false;
        *next_tick_time = None;
        // Wait before trying to acquire again.
        time::sleep(self.non_leader_acquisition_retry_interval).await;
    }

    /// Handles a non-specific error during the leader lock refresh attempt.
    async fn handle_refresh_error(
        &self,
        is_leader: &mut bool,
        next_tick_time: &mut Option<Instant>,
        error: LeaderElectionError,
    ) {
        tracing::error!(error = %error, "Failed to refresh leader lock due to an error. Relinquishing leadership as a precaution.");
        *is_leader = false;
        *next_tick_time = None;
        // Attempt to release the lock gracefully, but ignore the error as we might not hold it.
        if let Err(release_err) = self.leader_elector.release().await {
            tracing::warn!(error = %release_err,"Failed to release leader lock after refresh error (might have already lost it).");
        }
        // Wait before trying to acquire again, as the underlying issue might persist.
        time::sleep(self.non_leader_acquisition_retry_interval).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::{
            admin::bootstrap::BootstrapAdminUseCase,
            economy::bootstrap::BootstrapEconomyUseCase,
            ports::{
                crypto::MockPasswordHandler,
                downloader::MockBackupDownloader,
                init::{FlagKey, MockInitializationRepository},
                leader::MockLeaderElector,
                migration::MockMigrationRunner,
                processor::MockGameTickProcessable,
                restorer::MockDatabaseRestorer,
                uow::MockUnitOfWork,
            },
        },
        config::ServerConfig,
    };
    use anyhow::anyhow;
    use mockall::{predicate::*, Sequence};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time;

    // --- Test Constants ---
    const TICK_INTERVAL: Duration = Duration::from_millis(100);
    const REFRESH_INTERVAL: Duration = Duration::from_millis(1000);
    const RETRY_INTERVAL: Duration = Duration::from_millis(500);
    const INSTANCE_ID: &str = "test_instance_1";

    struct BuildManagerProps {
        elector: MockLeaderElector,
        processor: MockGameTickProcessable,
        uow: MockUnitOfWork,
        init_repo: MockInitializationRepository,
        restorer: MockDatabaseRestorer,
        downloader: MockBackupDownloader,
        pw_handler: MockPasswordHandler,
        migrator: MockMigrationRunner,
    }

    // --- Test Setup Helper ---
    fn build_manager_with_mocks(
        props: BuildManagerProps,
    ) -> LeaderLoopManager<
        MockLeaderElector,
        MockGameTickProcessable,
        MockUnitOfWork,
        MockInitializationRepository,
        MockDatabaseRestorer,
        MockBackupDownloader,
        MockPasswordHandler,
        MockMigrationRunner,
    > {
        let elector = Arc::new(props.elector);
        let processor = Arc::new(props.processor);
        let migrator_arc = Arc::new(props.migrator);
        let init_repo_arc = Arc::new(props.init_repo);
        let restorer = Arc::new(props.restorer);
        let downloader = Arc::new(props.downloader);
        let uow_arc = Arc::new(props.uow);
        let pw_handler_arc = Arc::new(props.pw_handler);

        let config = Arc::new(ServerConfig::default());

        let bootstrap_admin_uc = Arc::new(
            BootstrapAdminUseCase::builder()
                .uow(uow_arc.clone())
                .init_repo(init_repo_arc.clone())
                .pw(pw_handler_arc.clone())
                .build(),
        );

        let bootstrap_economy_uc = Arc::new(
            BootstrapEconomyUseCase::builder()
                .uow(uow_arc)
                .init_repo(init_repo_arc.clone())
                .config(config.clone())
                .build(),
        );

        let orchestrator = Arc::new(
            InitializationOrchestrator::builder()
                .config(config)
                .migrator(migrator_arc)
                .init_repo(init_repo_arc)
                .restorer(restorer)
                .downloader(downloader)
                .bootstrap_admin_uc(bootstrap_admin_uc)
                .bootstrap_economy_uc(bootstrap_economy_uc)
                .build(),
        );

        LeaderLoopManager::builder()
            .leader_elector(elector)
            .game_tick_processor(processor)
            .initialization_orchestrator(orchestrator)
            .instance_id(INSTANCE_ID.to_string())
            .leader_lock_refresh_interval(REFRESH_INTERVAL)
            .non_leader_acquisition_retry_interval(RETRY_INTERVAL)
            .game_tick_interval(TICK_INTERVAL)
            .build()
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn leader_runs_initialization_once_then_processes_ticks() {
        // ARRANGE
        let mut elector = MockLeaderElector::new();
        let mut processor = MockGameTickProcessable::new();
        let mut migrator = MockMigrationRunner::new();
        let mut init_repo = MockInitializationRepository::new();

        let uow = MockUnitOfWork::new();
        let restorer = MockDatabaseRestorer::new();
        let downloader = MockBackupDownloader::new();
        let pw_handler = MockPasswordHandler::new();

        let mut seq = Sequence::new();

        elector
            .expect_try_acquire()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(true));

        elector
            .expect_refresh()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(()));

        migrator
            .expect_run_migration()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(()));

        init_repo
            .expect_is_flag_set()
            .withf(|key| key.to_string() == FlagKey::Database.to_string())
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Ok(false));

        init_repo
            .expect_set_flag()
            .withf(|key| key.to_string() == FlagKey::Database.to_string())
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Ok(()));

        elector.expect_refresh().returning(|| Ok(()));

        processor
            .expect_process_next_tick()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(1));
        processor
            .expect_process_next_tick()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(2));

        let props = BuildManagerProps {
            elector,
            processor,
            uow,
            init_repo,
            restorer,
            downloader,
            pw_handler,
            migrator,
        };

        let manager = build_manager_with_mocks(props);

        let run_handle = tokio::spawn(manager.run());
        time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        time::advance(TICK_INTERVAL).await;
        tokio::task::yield_now().await;
        time::advance(TICK_INTERVAL).await;
        tokio::task::yield_now().await;
        run_handle.abort();
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn leader_relinquishes_on_initialization_failure() {
        // ARRANGE
        let mut elector = MockLeaderElector::new();
        let mut migrator = MockMigrationRunner::new();

        let processor = MockGameTickProcessable::new();
        let init_repo = MockInitializationRepository::new();
        let uow = MockUnitOfWork::new();
        let restorer = MockDatabaseRestorer::new();
        let downloader = MockBackupDownloader::new();
        let pw_handler = MockPasswordHandler::new();

        let mut seq = Sequence::new();

        // Initial acquisition succeeds
        elector
            .expect_try_acquire()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(true));

        // Refresh succeeds (leader state)
        elector
            .expect_refresh()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(()));

        // Migration fails during initialization
        migrator
            .expect_run_migration()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Err(anyhow!("DB migration failed").into()));

        // Leader releases lock due to initialization failure
        elector
            .expect_release()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(()));

        // After releasing, the loop continues as non-leader
        // Set up expectations for subsequent acquisition attempts to prevent infinite loop
        // We don't need this in the sequence since we just want to prevent re-acquiring
        elector.expect_try_acquire().returning(|| Ok(false)); // Always fail to acquire, preventing re-entering leader state

        let props = BuildManagerProps {
            elector,
            processor,
            uow,
            init_repo,
            restorer,
            downloader,
            pw_handler,
            migrator,
        };

        let manager = build_manager_with_mocks(props);

        // ACT & ASSERT
        let run_handle = tokio::spawn(manager.run());

        // Give enough time for the initialization failure to occur
        time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;

        // Advance time to allow the retry logic to execute
        time::advance(RETRY_INTERVAL).await;
        tokio::task::yield_now().await;

        // Clean shutdown
        run_handle.abort();
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn leader_skips_init_if_already_done_and_starts_ticking() {
        // ARRANGE
        let mut elector = MockLeaderElector::new();
        let mut processor = MockGameTickProcessable::new();
        let mut migrator = MockMigrationRunner::new();
        let mut init_repo = MockInitializationRepository::new();

        let uow = MockUnitOfWork::new();
        let restorer = MockDatabaseRestorer::new();
        let downloader = MockBackupDownloader::new();
        let pw_handler = MockPasswordHandler::new();

        elector.expect_try_acquire().times(1).returning(|| Ok(true));
        elector.expect_refresh().returning(|| Ok(()));

        migrator
            .expect_run_migration()
            .times(1)
            .returning(|| Ok(()));

        init_repo
            .expect_is_flag_set()
            .withf(|key| key.to_string() == FlagKey::Database.to_string())
            .times(1)
            .returning(|_| Ok(true));

        processor
            .expect_process_next_tick()
            .times(1)
            .returning(|| Ok(1));

        let props = BuildManagerProps {
            elector,
            processor,
            uow,
            init_repo,
            restorer,
            downloader,
            pw_handler,
            migrator,
        };

        let manager = build_manager_with_mocks(props);

        let run_handle = tokio::spawn(manager.run());
        time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        time::advance(TICK_INTERVAL).await;
        tokio::task::yield_now().await;
        run_handle.abort();
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn non_leader_waits_and_does_not_initialize() {
        // ARRANGE
        let mut elector = MockLeaderElector::new();
        let mut migrator = MockMigrationRunner::new();
        let mut init_repo = MockInitializationRepository::new();

        let processor = MockGameTickProcessable::new();
        let uow = MockUnitOfWork::new();
        let restorer = MockDatabaseRestorer::new();
        let downloader = MockBackupDownloader::new();
        let pw_handler = MockPasswordHandler::new();

        let mut seq = Sequence::new();

        elector
            .expect_try_acquire()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(false));

        elector
            .expect_try_acquire()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(true));

        elector
            .expect_refresh()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(()));
        migrator
            .expect_run_migration()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(()));

        init_repo
            .expect_is_flag_set()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Ok(false));
        init_repo
            .expect_set_flag()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Ok(()));

        let props = BuildManagerProps {
            elector,
            processor,
            uow,
            init_repo,
            restorer,
            downloader,
            pw_handler,
            migrator,
        };

        let manager = build_manager_with_mocks(props);

        let run_handle = tokio::spawn(manager.run());
        time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        time::advance(RETRY_INTERVAL).await;
        tokio::task::yield_now().await;
        run_handle.abort();
    }
}
