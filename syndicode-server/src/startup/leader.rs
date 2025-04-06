use crate::application::ports::leader::{LeaderElectionError, LeaderElector};
use std::{sync::Arc, time::Duration};
use tokio::time;

pub async fn run_leader_election_loop<L>(
    leader_elector: Arc<L>,
    instance_id: String,
    leader_lock_refresh_interval: Duration,
    non_leader_retry_acquisition_internal: Duration,
) where
    L: LeaderElector,
{
    let mut is_leader = false;

    loop {
        if is_leader {
            // --- Currently Leader ---
            // Try to refresh the lock
            match leader_elector.refresh().await {
                Ok(()) => {
                    // Still leader, continue doing leader work
                    tracing::debug!("Leader lock refreshed successfully.");
                    // *** DO LEADER WORK HERE (e.g., advance game tick) ***
                    // await advance_game_tick();
                    time::sleep(leader_lock_refresh_interval).await; // Wait before next refresh/work cycle
                }
                Err(LeaderElectionError::NotHoldingLock { .. }) => {
                    tracing::info!("Lost leadership or lock expired.");
                    is_leader = false;
                    // No sleep here, immediately try to re-acquire in the next loop iteration
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to refresh leader lock: {}. Relinquishing leadership.",
                        e
                    );
                    is_leader = false;
                    // Optional: Attempt graceful release, but might fail if connection is bad
                    // let _ = elector.release().await;
                    time::sleep(non_leader_retry_acquisition_internal).await; // Wait before retrying acquisition
                }
            }
        } else {
            // --- Not Currently Leader ---
            // Try to acquire the lock
            match leader_elector.try_acquire().await {
                Ok(true) => {
                    tracing::info!(
                        "Successfully acquired leadership (Instance ID: {})!",
                        instance_id
                    );
                    is_leader = true;
                    // No sleep, immediately enter the leader loop block in the next iteration
                }
                Ok(false) => {
                    tracing::trace!("Failed to acquire leadership, lock held by another instance.");
                    time::sleep(non_leader_retry_acquisition_internal).await; // Wait before retrying
                }
                Err(e) => {
                    tracing::error!("Error trying to acquire leader lock: {}", e);
                    time::sleep(non_leader_retry_acquisition_internal).await; // Wait before retrying
                }
            }
        }
    }
}
