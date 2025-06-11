use super::ValkeyStore;
use crate::application::ports::outcome::{
    OutcomeError, OutcomeNotifier, OutcomeResult, OutcomeStoreReader, OutcomeStoreWriter,
};
use redis::AsyncCommands;
use std::time::Duration;
use uuid::Uuid;

const PAYLOAD_KEY: &str = "syndicode:outcomes:payload";
const CLIENT_KEY: &str = "syndicode:outcomes:client";

const OUTCOME_TTL: Duration = Duration::from_secs(60);

pub const GAME_TICK_NOTIFICATION_CHANNEL: &str = "syndicode:game_tick";

#[tonic::async_trait]
impl OutcomeStoreWriter for ValkeyStore {
    async fn store_outcome(&self, request_uuid: Uuid, payload: &[u8]) -> OutcomeResult<()> {
        let key = format!("{PAYLOAD_KEY}:{}", request_uuid);

        let mut conn = self.conn.clone();

        // Explicitly specify the return type RV as ()
        // Infer K (key type) and V (value type)
        conn.set_ex::<_, _, ()>(key, payload, OUTCOME_TTL.as_secs())
            .await
            .map_err(|err| OutcomeError::EnqueueFailed(err.to_string()))?;

        Ok(())
    }
}

#[tonic::async_trait]
impl OutcomeStoreReader for ValkeyStore {
    async fn retrieve_outcome(&self, request_uuid: Uuid) -> OutcomeResult<Option<Vec<u8>>> {
        let key = format!("{PAYLOAD_KEY}:{}", request_uuid);

        let mut conn = self.conn.clone();

        let result: Option<Vec<u8>> = conn
            .get(key)
            .await
            .map_err(|err| OutcomeError::DequeueFailed(err.to_string()))?;

        Ok(result)
    }

    async fn delete_outcome(&self, request_uuid: Uuid) -> OutcomeResult<()> {
        let key = format!("{PAYLOAD_KEY}:{}", request_uuid);

        let mut conn = self.conn.clone();

        conn.del::<_, usize>(key)
            .await
            .map_err(|err| OutcomeError::DeletionFailed(err.to_string()))?;

        Ok(())
    }
}

#[tonic::async_trait]
impl OutcomeNotifier for ValkeyStore {
    async fn notify_outcome_ready(&self, user_uuid: Uuid, request_uuid: Uuid) -> OutcomeResult<()> {
        let channel_name = create_notification_channel(user_uuid);

        let mut conn = self.conn.clone();

        conn.publish::<_, _, usize>(channel_name, request_uuid.to_string())
            .await
            .map_err(|err| OutcomeError::PublishingOutcomeFailed(err.to_string()))?;

        Ok(())
    }

    async fn notify_game_tick_advanced(&self, game_tick: i64) -> OutcomeResult<()> {
        let mut conn = self.conn.clone();

        conn.publish::<_, _, usize>(GAME_TICK_NOTIFICATION_CHANNEL, game_tick)
            .await
            .map_err(|err| OutcomeError::PublishingGametickFailed(err.to_string()))?;

        Ok(())
    }
}

pub fn create_notification_channel(user_uuid: Uuid) -> String {
    format!("{CLIENT_KEY}:{}", user_uuid)
}
