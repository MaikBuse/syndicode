use crate::application::ports::leader::{LeaderElectionError, LeaderElector}; // Assume these are defined
use crate::application::ports::processor::{GameTickProcessable, ProcessorError};
use std::{sync::Arc, time::Duration};
use tokio::time::{self, Instant};

/// Manages the leader election loop and triggers the game tick processor when leader.
pub struct LeaderLoopManager<L, G>
where
    L: LeaderElector,
    G: GameTickProcessable,
{
    leader_elector: Arc<L>,
    game_tick_processor: Arc<G>,
    instance_id: String,
    leader_lock_refresh_interval: Duration,
    non_leader_acquisition_retry_interval: Duration,
    game_tick_interval: Duration,
}

impl<L, G> LeaderLoopManager<L, G>
where
    L: LeaderElector + Send + Sync + 'static,
    G: GameTickProcessable + Send + Sync + 'static,
{
    pub fn new(
        leader_elector: Arc<L>,
        game_tick_processor: Arc<G>,
        instance_id: String,
        leader_lock_refresh_interval: Duration,
        non_leader_acquisition_retry_interval: Duration,
        game_tick_interval: Duration,
    ) -> Self {
        Self {
            leader_elector,
            game_tick_processor,
            instance_id,
            leader_lock_refresh_interval,
            non_leader_acquisition_retry_interval,
            game_tick_interval,
        }
    }

    /// Runs the leader election loop indefinitely
    #[allow(clippy::cognitive_complexity)] // Okay for the main loop structure
    pub async fn run(self) {
        let mut is_leader = false;
        // Tracks when the *next* tick processing cycle should begin.
        let mut next_tick_time: Option<Instant> = None;

        loop {
            if is_leader {
                // --- Currently Leader ---
                // We refresh periodically based on sleep logic below.
                match self.leader_elector.refresh().await {
                    Ok(()) => {
                        // Still leader. Drive the fixed tick loop.
                        tracing::trace!("Leader lock refreshed successfully.");

                        // Initialize next_tick_time if this is the first time after becoming leader
                        // or after a period of being behind schedule.
                        if next_tick_time.is_none() {
                            let first_tick_target = Instant::now() + self.game_tick_interval;
                            next_tick_time = Some(first_tick_target);
                            tracing::info!(
                                "Initialized tick timer. First tick target: {:?}",
                                first_tick_target
                            );
                        }
                        // We must unwrap here; if it was None, it was just set.
                        let mut current_tick_target = next_tick_time.unwrap();

                        // Inner loop to process potentially multiple ticks if behind schedule.
                        loop {
                            let now = Instant::now();

                            // Check if it's time for the next scheduled tick
                            if now >= current_tick_target {
                                let tick_start_offset = now - current_tick_target;
                                tracing::debug!(
                                    lag_ms = tick_start_offset.as_millis(),
                                    "Starting tick processing for target time: {:?}",
                                    current_tick_target
                                );

                                // *** Process the actual game tick ***
                                match self.game_tick_processor.process_next_tick().await {
                                    Ok(processed_tick) => {
                                        let duration = Instant::now() - now;
                                        tracing::info!(
                                            tick = processed_tick,
                                            duration_ms = duration.as_millis(),
                                            target_interval_ms =
                                                self.game_tick_interval.as_millis(),
                                            lag_ms = tick_start_offset.as_millis(),
                                            "Successfully processed game tick."
                                        );

                                        // Handle potential overruns where processing took longer than the interval
                                        if duration > self.game_tick_interval {
                                            tracing::warn!(
                                                  duration_ms = duration.as_millis(),
                                                  target_ms = self.game_tick_interval.as_millis(),
                                                  "Tick processing duration exceeded target interval!"
                                             );
                                            // The scheduling logic below handles catch-up.
                                        }

                                        // *** Schedule the next tick ***
                                        // Always advance the target time by the fixed interval,
                                        // regardless of processing duration, to maintain a stable tick rate.
                                        current_tick_target += self.game_tick_interval;
                                        next_tick_time = Some(current_tick_target); // Update shared state

                                        // If the *new* target time is still in the past, we are behind schedule.
                                        // Loop again immediately to process the next tick.
                                        if current_tick_target <= Instant::now() {
                                            tracing::warn!("System is behind schedule. Processing next tick immediately.");
                                            // continue loop; // Implicitly continues
                                        } else {
                                            // The next tick is in the future. Break the inner loop to sleep.
                                            break;
                                        }
                                    }
                                    // --- Start of modified section ---
                                    Err(err) => {
                                        match err {
                                            // Check if it's the specific NotInitialized error
                                            ProcessorError::NotInitialized => {
                                                tracing::warn!("Tick processing skipped: Database not initialized yet. Will retry on next cycle.");
                                                // *** IMPORTANT: Do NOT relinquish leadership ***
                                                // Allow the outer loop's sleep logic to handle the retry timing.
                                                // We break the inner loop because we can't proceed *this* tick.
                                                break; // Break inner loop, go to outer loop sleep/refresh check
                                            }
                                            // Handle other known critical ProcessorErrors if desired
                                            _ => {
                                                tracing::error!("Game tick processing failed (ProcessorError: {}). Relinquishing leadership.", err);
                                                // Treat other ProcessorErrors as critical: Relinquish leadership
                                                if let Err(release_err) =
                                                    self.leader_elector.release().await
                                                {
                                                    tracing::error!(error = %release_err, "Failed to release leader lock after critical processing error.");
                                                }
                                                is_leader = false;
                                                next_tick_time = None;
                                                // Wait before trying acquire again for critical errors
                                                time::sleep(
                                                    self.non_leader_acquisition_retry_interval,
                                                )
                                                .await;
                                                break; // Break inner tick loop, go to outer acquire loop
                                            }
                                        }
                                    } // --- End of modified section ---
                                } // End match process_next_tick
                            } else {
                                // Not time for the current target tick yet. Break inner loop to sleep.
                                break;
                            }
                        } // --- End of inner tick processing loop ---

                        // If we are still leader after the inner loop (i.e., no critical processing error occurred)...
                        if is_leader {
                            // Calculate sleep duration. We need to wake up for the *earlier* of:
                            // 1. The next scheduled game tick (`next_tick_time`).
                            // 2. The next required lock refresh time.
                            let now = Instant::now();
                            // next_tick_time should be Some here, as it's set after successful processing
                            // or initialization, or we broke inner loop due to NotInitialized.
                            // If it somehow became None (e.g., critical error path, but is_leader is true?), default to now.
                            let next_tick_due_at = next_tick_time.unwrap_or(now);

                            let time_until_next_tick = if next_tick_due_at > now {
                                next_tick_due_at - now
                            } else {
                                Duration::ZERO // Already due or past due
                            };

                            // Calculate time until the next refresh *check*. Check slightly before expiry.
                            // Using 90% of the interval as a safety margin.
                            let time_until_refresh_needed =
                                self.leader_lock_refresh_interval.mul_f32(0.9);

                            // Sleep until the *minimum* of the two durations.
                            let sleep_duration =
                                time_until_next_tick.min(time_until_refresh_needed);

                            if sleep_duration > Duration::ZERO {
                                tracing::trace!(
                                    "Sleeping for {:?} (until next tick: {:?}, until refresh: {:?})",
                                    sleep_duration, time_until_next_tick, time_until_refresh_needed
                                );
                                time::sleep(sleep_duration).await;
                            } else {
                                // If sleep duration is zero (e.g., we are behind schedule or refresh is due),
                                // yield to allow other tasks to run and prevent hogging CPU.
                                tracing::trace!(
                                    "Calculated sleep duration is zero or negative. Yielding."
                                );
                                tokio::task::yield_now().await;
                            }
                        }
                        // If !is_leader (due to critical processing error), the outer loop will handle the transition.
                    } // End Ok(()) refresh case
                    Err(LeaderElectionError::NotHoldingLock { key, instance_id }) => {
                        // This can happen if the lock expired between the last refresh and this one,
                        // or if another instance took over.
                        tracing::info!(key=%key, current_instance=%self.instance_id , owner_instance=%instance_id, "Lost leadership or lock expired (refresh check failed).");
                        is_leader = false;
                        next_tick_time = None; // Reset timer state
                                               // No sleep here, immediately try to re-acquire in the next outer loop iteration.
                    }
                    Err(e) => {
                        // This indicates an error communicating with the lock provider (e.g., network issue).
                        // It's safer to assume we might have lost leadership or might lose it soon.
                        tracing::error!(error = %e, "Failed to refresh leader lock due to an error. Relinquishing leadership as a precaution.");
                        is_leader = false;
                        next_tick_time = None; // Reset timer state
                                               // Attempt to release the lock gracefully, but ignore error as we might not hold it.
                        if let Err(release_err) = self.leader_elector.release().await {
                            tracing::warn!(error = %release_err,"Failed to release leader lock after refresh error (might have already lost it).");
                        }
                        // Wait before trying to acquire again, as the underlying issue might persist.
                        time::sleep(self.non_leader_acquisition_retry_interval).await;
                    }
                } // End match leader_elector.refresh()
            } else {
                // --- Not Currently Leader ---
                tracing::debug!("Not leader. Attempting to acquire lock...");
                match self.leader_elector.try_acquire().await {
                    Ok(true) => {
                        tracing::info!(instance_id = %self.instance_id, "Successfully acquired leadership!");
                        is_leader = true;
                        next_tick_time = None; // Ensure timer gets re-initialized on first leader cycle
                                               // No sleep, immediately loop back to start leader duties (refresh/tick)
                    }
                    Ok(false) => {
                        // Failed to acquire, someone else is leader or lock unavailable. Wait before retrying.
                        tracing::debug!("Failed to acquire lock (already held or unavailable). Retrying after interval.");
                        time::sleep(self.non_leader_acquisition_retry_interval).await;
                    }
                    Err(e) => {
                        // Error during acquisition attempt (e.g., network issue). Wait before retrying.
                        tracing::error!(error = %e, "Error trying to acquire leader lock. Retrying after interval.");
                        time::sleep(self.non_leader_acquisition_retry_interval).await;
                    }
                }
            } // End if is_leader / else
        } // End outer loop
    } // End run()
}

#[cfg(test)]
mod tests {
    use crate::application::ports::processor::ProcessorResult;

    use super::*;
    use anyhow::anyhow;
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tokio::time::{self}; // Keep Instant import

    // --- Mock State Structures ---

    #[derive(Debug, Clone, PartialEq)]
    enum MockLeaderAction {
        TryAcquire,
        Refresh,
        Release,
    }

    #[derive(Debug)] // Add Debug for easier tracing/logging
    struct MockLeaderState {
        is_held: bool,
        // Use Option<> for results to allow consuming them once
        acquire_results: VecDeque<Result<bool, LeaderElectionError>>,
        refresh_results: VecDeque<Result<(), LeaderElectionError>>,
        release_results: VecDeque<Result<(), LeaderElectionError>>,
        actions_called: Vec<MockLeaderAction>,
        instance_id: String, // Track which instance holds the "lock"
    }

    struct MockGameTickState {
        process_results: VecDeque<Result<i64, anyhow::Error>>,
        ticks_processed: Vec<i64>,
        errors_encountered: Vec<String>,
        call_count: i64,
    }

    // --- Mock Implementations ---

    #[derive(Clone)]
    struct MockLeaderElector {
        state: Arc<Mutex<MockLeaderState>>,
        mock_instance_id: String, // ID of this mock instance
    }

    impl MockLeaderElector {
        fn new(instance_id: &str) -> Self {
            Self {
                state: Arc::new(Mutex::new(MockLeaderState {
                    is_held: false,
                    acquire_results: VecDeque::new(),
                    refresh_results: VecDeque::new(),
                    release_results: VecDeque::new(),
                    actions_called: Vec::new(),
                    instance_id: "".to_string(), // Initially no one holds it
                })),
                mock_instance_id: instance_id.to_string(),
            }
        }

        // Helper to queue results
        fn add_acquire_result(&self, result: Result<bool, LeaderElectionError>) {
            self.state.lock().unwrap().acquire_results.push_back(result);
        }
        fn add_refresh_result(&self, result: Result<(), LeaderElectionError>) {
            self.state.lock().unwrap().refresh_results.push_back(result);
        }
        fn add_release_result(&self, result: Result<(), LeaderElectionError>) {
            self.state.lock().unwrap().release_results.push_back(result);
        }

        fn get_actions(&self) -> Vec<MockLeaderAction> {
            self.state.lock().unwrap().actions_called.clone()
        }
        fn clear_actions(&self) {
            self.state.lock().unwrap().actions_called.clear();
        }
        // Helper to check mock state accurately
        fn is_held_by_mock(&self) -> bool {
            let state = self.state.lock().unwrap();
            // Add tracing for debugging test failures
            // tracing::trace!(instance=%self.mock_instance_id, state_is_held=state.is_held, state_instance_id=%state.instance_id, check_result=held, "is_held_by_mock check");
            state.is_held && state.instance_id == self.mock_instance_id
        }
    }

    #[tonic::async_trait] // Use direct async_trait
    impl LeaderElector for MockLeaderElector {
        async fn try_acquire(&self) -> Result<bool, LeaderElectionError> {
            let mut state = self.state.lock().unwrap();
            state.actions_called.push(MockLeaderAction::TryAcquire);

            // Consume a queued result or return default error if none
            let result = state.acquire_results.pop_front().unwrap_or_else(|| {
                // Default behavior: succeed only if no one holds it
                if !state.is_held {
                    Ok(true)
                } else {
                    Ok(false)
                }
            });

            tracing::trace!(instance=%self.mock_instance_id, is_held=state.is_held, holder=%state.instance_id, "Attempting acquire, queued result: {:?}", result);

            match &result {
                Ok(true) => {
                    // Acquire succeeded for this instance
                    state.is_held = true;
                    state.instance_id = self.mock_instance_id.clone();
                    tracing::info!(instance=%self.mock_instance_id, "Acquire successful, updated mock state.");
                    // Clear subsequent results unless overridden by test
                    state.refresh_results.clear();
                    state.release_results.clear();
                }
                Ok(false) => {
                    // Failed to acquire, don't change held status unless this instance held it erroneously
                    if state.is_held && state.instance_id == self.mock_instance_id {
                        tracing::warn!(instance=%self.mock_instance_id, "Acquire returned false, but mock thought it held the lock. Correcting state.");
                        state.is_held = false;
                        state.instance_id = "".to_string();
                    } else {
                        tracing::trace!(instance=%self.mock_instance_id, "Acquire failed (Ok(false)).");
                    }
                }
                Err(_) => {
                    // Error during acquire, ensure this instance doesn't hold it
                    if state.instance_id == self.mock_instance_id {
                        tracing::warn!(instance=%self.mock_instance_id, "Acquire failed (Error). Correcting state.");
                        state.is_held = false;
                        state.instance_id = "".to_string();
                    } else {
                        tracing::trace!(instance=%self.mock_instance_id, "Acquire failed (Error), was not holder.");
                    }
                }
            }
            let final_result = result.clone(); // Clone before dropping state guard
            drop(state);
            final_result
        }

        async fn refresh(&self) -> Result<(), LeaderElectionError> {
            let mut state = self.state.lock().unwrap();
            state.actions_called.push(MockLeaderAction::Refresh);

            let current_holder = state.instance_id.clone();
            let currently_held = state.is_held;
            // Check if *this* instance is the current holder in the mock state
            let am_holder = currently_held && current_holder == self.mock_instance_id;

            tracing::trace!(instance=%self.mock_instance_id, is_held=currently_held, holder=%current_holder, am_holder=am_holder, "Enter refresh mock");

            // If this instance doesn't hold the lock according to state, return error immediately
            if !am_holder {
                tracing::warn!(instance=%self.mock_instance_id, "Refresh called but mock state indicates not holding lock.");
                // Consume a potential queued result *anyway*, otherwise return default error
                let queued_result = state.refresh_results.pop_front();
                let result = queued_result.unwrap_or_else(|| {
                    Err(LeaderElectionError::NotHoldingLock {
                        key: "mock_key".to_string(),
                        instance_id: current_holder, // Report who (if anyone) holds it
                    })
                });
                drop(state);
                return result;
            }

            // This instance believes it holds the lock. Consume a queued result or default to Ok(())
            let result = state.refresh_results.pop_front().unwrap_or(Ok(()));
            tracing::trace!(instance=%self.mock_instance_id, "Consumed refresh result: {:?}", result);

            // Update mock state based on the refresh outcome *only if this instance was the holder*
            match &result {
                Ok(()) => {
                    // Refresh succeeded, state remains held by this instance
                    tracing::trace!(instance=%self.mock_instance_id, "Refresh successful, state remains held.");
                }
                Err(e) => {
                    // Refresh failed. Update mock state to reflect loss of leadership for THIS instance.
                    tracing::info!(instance=%self.mock_instance_id, error=%e, "Refresh failed, updating mock state to not held.");
                    state.is_held = false;
                    state.instance_id = "".to_string();
                    // Set default refresh error for subsequent calls if needed
                    if state.refresh_results.is_empty() {
                        state
                            .refresh_results
                            .push_back(Err(LeaderElectionError::NotHoldingLock {
                                key: "mock_key".to_string(),
                                instance_id: "".to_string(),
                            }));
                    }
                }
            }
            let final_result = result.clone(); // Clone before dropping lock
            drop(state);
            final_result
        }

        async fn release(&self) -> Result<(), LeaderElectionError> {
            let mut state = self.state.lock().unwrap();
            // IMPORTANT: Always record the action, even if release is called when not holding
            state.actions_called.push(MockLeaderAction::Release);

            let current_holder = state.instance_id.clone();
            let currently_held = state.is_held;
            // Check if *this* instance is the current holder in the mock state
            let am_holder = currently_held && current_holder == self.mock_instance_id;

            tracing::trace!(instance=%self.mock_instance_id, is_held=currently_held, holder=%current_holder, am_holder=am_holder, "Enter release mock");

            // Consume a queued result or default to Ok(())
            let result = state.release_results.pop_front().unwrap_or(Ok(()));
            tracing::trace!(instance=%self.mock_instance_id, "Consumed release result: {:?}", result);

            // Regardless of result, if this instance WAS the holder, mark as not held
            if am_holder {
                tracing::trace!(instance=%self.mock_instance_id, "Releasing lock held by this instance.");
                state.is_held = false;
                state.instance_id = "".to_string();
            } else {
                tracing::trace!(instance=%self.mock_instance_id, "Release called but lock not held by it.");
                // Optional: Correct state if somehow inconsistent (e.g., instance_id matches but is_held is false)
                if state.instance_id == self.mock_instance_id {
                    state.is_held = false; // Ensure consistency
                }
            }

            // Set default refresh error since lock is now released (or wasn't held)
            state.refresh_results.clear(); // Clear any pending refresh results after release
            state
                .refresh_results
                .push_back(Err(LeaderElectionError::NotHoldingLock {
                    key: "mock_key".to_string(),
                    instance_id: "".to_string(), // No holder after release
                }));

            let final_result = result.clone(); // Clone before dropping lock
            drop(state);
            final_result
        }
    }

    #[derive(Clone)]
    struct MockGameTickProcessor {
        state: Arc<Mutex<MockGameTickState>>,
    }

    impl MockGameTickProcessor {
        fn new() -> Self {
            Self {
                state: Arc::new(Mutex::new(MockGameTickState {
                    process_results: VecDeque::new(),
                    ticks_processed: Vec::new(),
                    errors_encountered: Vec::new(),
                    call_count: 0,
                })),
            }
        }
        fn add_process_result(&self, result: Result<i64, anyhow::Error>) {
            self.state.lock().unwrap().process_results.push_back(result);
        }
        fn get_processed_ticks(&self) -> Vec<i64> {
            self.state.lock().unwrap().ticks_processed.clone()
        }
        fn get_call_count(&self) -> i64 {
            self.state.lock().unwrap().call_count
        }
        fn get_errors(&self) -> Vec<String> {
            self.state.lock().unwrap().errors_encountered.clone()
        }
    }

    #[tonic::async_trait]
    impl GameTickProcessable for MockGameTickProcessor {
        async fn process_next_tick(&self) -> ProcessorResult<i64> {
            // --- Start Lock ---
            let mut state = self.state.lock().unwrap();
            state.call_count += 1;

            // Take the result from the queue or create default
            let result = state
                .process_results
                .pop_front()
                .unwrap_or_else(|| Ok(state.call_count)); // This is anyhow::Result<usize>

            // Prepare the value to be returned *after* the lock is dropped
            #[allow(clippy::needless_late_init)]
            let outcome: ProcessorResult<i64>;

            // Process the result *inside* the lock to update state
            match result {
                Ok(tick_value) => {
                    state.ticks_processed.push(tick_value);
                    // Prepare Ok outcome
                    outcome = Ok(tick_value);
                }
                Err(e) => {
                    // Store string representation of the error for test assertions
                    state.errors_encountered.push(e.to_string());
                    // Prepare an Err outcome. We cannot return the original 'e',
                    // so we create a new, simple error. The LeaderLoopManager
                    // only cares *that* an error happened, not the specifics here.
                    outcome = Err(anyhow!("Mock tick processing failed").into());
                    // Generic error
                }
            };

            // --- End Lock (implicitly dropped by 'state' going out of scope) ---
            drop(state);

            // Return the prepared outcome
            outcome
        }
    }

    // --- Test Setup Helper ---
    const TICK_INTERVAL_MS: u64 = 100;
    const REFRESH_INTERVAL_MS: u64 = 1000;
    const RETRY_INTERVAL_MS: u64 = 500;
    const INSTANCE_ID: &str = "test_instance_1";

    fn setup_manager() -> (
        Arc<MockLeaderElector>,
        Arc<MockGameTickProcessor>,
        LeaderLoopManager<MockLeaderElector, MockGameTickProcessor>,
    ) {
        // init_tracing(); // Enable tracing here for debugging specific tests if needed
        let elector = Arc::new(MockLeaderElector::new(INSTANCE_ID));
        let processor = Arc::new(MockGameTickProcessor::new());

        let manager = LeaderLoopManager::new(
            elector.clone(),
            processor.clone(),
            INSTANCE_ID.to_string(),
            Duration::from_millis(REFRESH_INTERVAL_MS), // Refresh interval
            Duration::from_millis(RETRY_INTERVAL_MS),   // Non-leader retry
            Duration::from_millis(TICK_INTERVAL_MS),    // Game tick interval
        );

        (elector, processor, manager)
    }

    // --- The Tests ---

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_acquires_leadership_and_processes_tick() {
        // init_tracing(); // Enable per-test if needed
        let (elector, processor, manager) = setup_manager();
        elector.add_acquire_result(Ok(true));
        processor.add_process_result(Ok(1));
        let run_handle = tokio::spawn(manager.run());

        time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await; // Acquire + Refresh
        assert!(
            elector.is_held_by_mock(),
            "Should hold leadership after acquire"
        );
        assert_eq!(elector.get_actions().len(), 2); // Acquire, Refresh
        elector.clear_actions();

        time::advance(Duration::from_millis(TICK_INTERVAL_MS)).await;
        tokio::task::yield_now().await; // Process tick + Refresh before sleep
        assert_eq!(processor.get_call_count(), 1, "Tick 1 count");
        assert_eq!(processor.get_processed_ticks(), vec![1], "Tick 1 value");
        assert!(
            elector.get_actions().contains(&MockLeaderAction::Refresh),
            "Refresh before sleep"
        );
        elector.clear_actions();

        time::advance(Duration::from_millis(TICK_INTERVAL_MS)).await;
        tokio::task::yield_now().await; // Process tick 2 + Refresh before sleep
        assert_eq!(processor.get_call_count(), 2, "Tick 2 count");
        assert_eq!(processor.get_processed_ticks(), vec![1, 2], "Tick 2 value");
        assert!(
            elector.get_actions().contains(&MockLeaderAction::Refresh),
            "Refresh before sleep"
        );

        run_handle.abort();
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_stays_leader_and_processes_multiple_ticks() {
        let (elector, processor, manager) = setup_manager();
        elector.add_acquire_result(Ok(true));
        processor.add_process_result(Ok(1));
        processor.add_process_result(Ok(2));
        processor.add_process_result(Ok(3));
        let run_handle = tokio::spawn(manager.run());

        time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await; // Acquire + Refresh
        assert!(elector.is_held_by_mock());
        assert_eq!(processor.get_call_count(), 0);

        time::advance(Duration::from_millis(TICK_INTERVAL_MS)).await; // Tick 1 + Refresh
        tokio::task::yield_now().await;
        assert_eq!(processor.get_call_count(), 1);
        assert_eq!(processor.get_processed_ticks(), vec![1]);

        time::advance(Duration::from_millis(TICK_INTERVAL_MS)).await; // Tick 2 + Refresh
        tokio::task::yield_now().await;
        assert_eq!(processor.get_call_count(), 2);
        assert_eq!(processor.get_processed_ticks(), vec![1, 2]);

        time::advance(Duration::from_millis(TICK_INTERVAL_MS)).await; // Tick 3 + Refresh
        tokio::task::yield_now().await;
        assert_eq!(processor.get_call_count(), 3);
        assert_eq!(processor.get_processed_ticks(), vec![1, 2, 3]);

        elector.clear_actions();
        time::advance(Duration::from_millis(REFRESH_INTERVAL_MS)).await; // Force refresh check + catch up ticks
        tokio::task::yield_now().await;
        assert!(processor.get_call_count() > 3);
        assert!(elector.get_actions().contains(&MockLeaderAction::Refresh));
        assert!(elector.is_held_by_mock());

        run_handle.abort();
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_handles_tick_processing_error_and_releases() {
        let (elector, processor, manager) = setup_manager();
        elector.add_acquire_result(Ok(true));
        processor.add_process_result(Err(anyhow!("DB fail")));
        elector.add_release_result(Ok(())); // Expect release

        let run_handle = tokio::spawn(manager.run());

        // Become leader (Acquire + Refresh)
        time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        assert!(elector.is_held_by_mock());
        elector.clear_actions();

        // Advance time to trigger the failing tick processing (needs refresh first)
        elector.add_refresh_result(Ok(())); // Add expected refresh
        time::advance(Duration::from_millis(TICK_INTERVAL_MS)).await;
        tokio::task::yield_now().await; // Hits Refresh, process_next_tick fails, calls release, sleeps

        assert_eq!(processor.get_call_count(), 1);
        assert_eq!(processor.get_processed_ticks().len(), 0);
        assert_eq!(processor.get_errors().len(), 1);

        assert!(
            !elector.is_held_by_mock(),
            "Not leader after processing error"
        );
        let actions = elector.get_actions();
        let refresh_pos = actions.iter().position(|a| *a == MockLeaderAction::Refresh);
        let release_pos = actions.iter().position(|a| *a == MockLeaderAction::Release);
        assert!(refresh_pos.is_some(), "Refresh should have happened");
        assert!(
            release_pos.is_some(),
            "Release should be called after processing error"
        );
        assert!(
            refresh_pos < release_pos,
            "Refresh must happen before Release"
        );

        // Check wait logic
        elector.clear_actions();
        time::advance(Duration::from_millis(RETRY_INTERVAL_MS - 1)).await;
        tokio::task::yield_now().await;
        assert!(elector.get_actions().is_empty(), "No acquire during wait");

        elector.add_acquire_result(Ok(false));
        time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        assert!(
            elector
                .get_actions()
                .contains(&MockLeaderAction::TryAcquire),
            "Acquire after wait"
        );

        run_handle.abort();
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_retries_acquisition_if_failed() {
        let (elector, processor, manager) = setup_manager();
        elector.add_acquire_result(Ok(false));
        elector.add_acquire_result(Ok(false));
        elector.add_acquire_result(Ok(true)); // Succeeds 3rd time
        let run_handle = tokio::spawn(manager.run());

        // Attempt 1 (fails)
        time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        assert_eq!(elector.get_actions(), vec![MockLeaderAction::TryAcquire]);
        assert!(!elector.is_held_by_mock());

        // Wait and Attempt 2 (fails)
        elector.clear_actions();
        time::advance(Duration::from_millis(RETRY_INTERVAL_MS + 10)).await;
        tokio::task::yield_now().await;
        assert_eq!(elector.get_actions(), vec![MockLeaderAction::TryAcquire]);
        assert!(!elector.is_held_by_mock());

        // Wait and Attempt 3 (succeeds + Refresh)
        elector.clear_actions();
        time::advance(Duration::from_millis(RETRY_INTERVAL_MS + 10)).await;
        tokio::task::yield_now().await;
        assert_eq!(
            elector.get_actions(),
            vec![MockLeaderAction::TryAcquire, MockLeaderAction::Refresh]
        );
        assert!(elector.is_held_by_mock());

        // Check tick processing starts correctly after becoming leader
        let processor_calls_before_tick = processor.get_call_count();
        assert_eq!(processor_calls_before_tick, 0);
        time::advance(Duration::from_millis(TICK_INTERVAL_MS - 1)).await;
        tokio::task::yield_now().await;
        assert_eq!(processor.get_call_count(), 0); // Not yet

        processor.add_process_result(Ok(1));
        time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await; // Refresh + Tick
        assert_eq!(processor.get_call_count(), 1);
        assert_eq!(processor.get_processed_ticks(), vec![1]);

        run_handle.abort();
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn test_tick_catch_up_logic() {
        let tick_interval = Duration::from_millis(50);
        let elector = Arc::new(MockLeaderElector::new(INSTANCE_ID));
        let processor = Arc::new(MockGameTickProcessor::new());
        let manager = LeaderLoopManager::new(
            elector.clone(),
            processor.clone(),
            INSTANCE_ID.to_string(),
            Duration::from_millis(1000),
            Duration::from_millis(500),
            tick_interval,
        );

        elector.add_acquire_result(Ok(true));
        processor.add_process_result(Ok(1));
        processor.add_process_result(Ok(2));
        processor.add_process_result(Ok(3));
        processor.add_process_result(Ok(4));
        let run_handle = tokio::spawn(manager.run());

        // Become leader (Acquire + Refresh)
        time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        assert!(elector.is_held_by_mock());
        elector.clear_actions();

        // Advance time for multiple ticks to be due (needs one refresh first)
        elector.add_refresh_result(Ok(())); // Add expected refresh
        let advance_duration = tick_interval * 3 + Duration::from_millis(5); // ~155ms
        time::advance(advance_duration).await;
        tokio::task::yield_now().await; // Runs Refresh, then catches up 3 ticks

        assert_eq!(processor.get_call_count(), 3, "Catch up 3 ticks");
        assert_eq!(processor.get_processed_ticks(), vec![1, 2, 3]);
        assert!(
            elector.get_actions().contains(&MockLeaderAction::Refresh),
            "Refresh before catchup"
        ); // Refresh should have happened
        elector.clear_actions();

        // Advance past the *next* tick interval (needs refresh first)
        elector.add_refresh_result(Ok(()));
        time::advance(tick_interval + Duration::from_millis(5)).await; // ~55ms
        tokio::task::yield_now().await; // Runs Refresh, then tick 4

        assert_eq!(processor.get_call_count(), 4, "Process 4th tick");
        assert_eq!(processor.get_processed_ticks(), vec![1, 2, 3, 4]);
        assert!(elector.get_actions().contains(&MockLeaderAction::Refresh));

        run_handle.abort();
    }
} // End tests module
