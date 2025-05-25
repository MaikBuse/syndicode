use std::sync::Arc;

use dashmap::DashMap;
use syndicode_proto::syndicode_interface_v1::GameUpdate;
use tokio::sync::mpsc;
use tonic::Status;
use uuid::Uuid;

// Type alias for the sender part of the channel for game updates to a specific user.
pub type UserTx = mpsc::Sender<Result<GameUpdate, Status>>;
// Type alias for the map storing user channels, keyed by user UUID.
// Stores Arc<UserTx> to allow UserChannelGuard to correctly identify and remove
// the specific channel instance it's responsible for.
pub type UserChannels = Arc<DashMap<Uuid, Arc<UserTx>>>;

/// RAII Guard for Channel Cleanup
pub struct UserChannelGuard {
    pub user_id: Uuid,
    pub channels: UserChannels, // The shared map of all user channels
    pub channel_instance: Arc<UserTx>, // The specific channel instance this guard manages
}

impl Drop for UserChannelGuard {
    fn drop(&mut self) {
        tracing::debug!(
            "UserChannelGuard drop: Attempting conditional remove_if for user {}.",
            self.user_id
        );

        // remove_if takes the key and a closure.
        // The closure receives the key and a mutable reference to the value.
        // It should return true if the entry should be removed, false otherwise.
        // The DashMap entry is locked during the execution of this closure.
        let removed_entry = self.channels.remove_if(&self.user_id, |_key, current_channel_in_map_ref| {
            // Compare the Arc<UserTx> instance in the map with the one this guard is responsible for.
            // We are comparing the pointers of the Arcs.
            if Arc::ptr_eq(current_channel_in_map_ref, &self.channel_instance) {
                // Pointers match, so this is the exact channel instance we should remove.
                tracing::debug!(
                    "UserChannelGuard (remove_if): Instance match for user {}. Marking for removal.",
                    self.user_id
                );
                true // Yes, remove this entry.
            } else {
                // Pointers do not match. This means the channel for this user_id
                // was likely replaced by a newer connection after this guard was created
                // but before it was dropped. We should not remove the newer channel.
                tracing::debug!(
                    "UserChannelGuard (remove_if): Instance mismatch for user {}. Not removing (likely replaced).",
                    self.user_id
                );
                false // No, do not remove this entry.
            }
        });

        // Log whether an entry was actually found and processed by remove_if
        if let Some((_removed_key, _removed_value_arc)) = removed_entry {
            tracing::debug!(
                "UserChannelGuard drop: Successfully processed and removed entry for user {} via remove_if.",
                self.user_id
            );
        } else {
            // This case means either:
            // 1. The user_id was not found in the map when remove_if was called.
            // 2. The user_id was found, but the closure returned `false` (e.g., Arc::ptr_eq was false).
            //    The tracing inside the closure should clarify which one.
            tracing::debug!(
                "UserChannelGuard drop: Entry for user {} was not removed by remove_if (either not found or closure returned false).",
                self.user_id
            );
        }
        tracing::debug!("UserChannelGuard drop: Finished for user {}.", self.user_id);
    }
}
