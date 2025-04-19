use super::ValkeyStore;
use crate::application::ports::results::{
    ResultNotifier, ResultResult, ResultStoreReader, ResultStoreWriter,
};
use redis::AsyncCommands;
use std::time::Duration;
use uuid::Uuid;

const PAYLOAD_KEY: &str = "syndicode:results:payload";
const CLIENT_KEY: &str = "syndicode:results:client";

const OUTCOME_TTL: Duration = Duration::from_secs(300);

#[tonic::async_trait]
impl ResultStoreWriter for ValkeyStore {
    async fn store_result(&self, request_uuid: Uuid, payload: &[u8]) -> ResultResult<()> {
        let key = format!("{PAYLOAD_KEY}:{}", request_uuid);

        let mut conn = self.conn.clone();

        // Explicitly specify the return type RV as ()
        // Infer K (key type) and V (value type)
        conn.set_ex::<_, _, ()>(key, payload, OUTCOME_TTL.as_secs())
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }
}

#[tonic::async_trait]
impl ResultStoreReader for ValkeyStore {
    async fn retrieve_result(&self, request_uuid: Uuid) -> ResultResult<Option<Vec<u8>>> {
        let key = format!("{PAYLOAD_KEY}:{}", request_uuid);

        let mut conn = self.conn.clone();

        let result: Option<Vec<u8>> = conn.get(key).await.map_err(anyhow::Error::from)?;

        Ok(result)
    }

    async fn delete_result(&self, request_uuid: Uuid) -> ResultResult<()> {
        let key = format!("{PAYLOAD_KEY}:{}", request_uuid);

        let mut conn = self.conn.clone();

        conn.del::<_, usize>(key)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }
}

#[tonic::async_trait]
impl ResultNotifier for ValkeyStore {
    async fn notify_result_ready(&self, user_uuid: Uuid, request_uuid: Uuid) -> ResultResult<()> {
        let channel_name = create_notification_channel(user_uuid);

        let mut conn = self.conn.clone();

        conn.publish::<_, _, usize>(channel_name, request_uuid.to_string())
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }
}

pub fn create_notification_channel(user_uuid: Uuid) -> String {
    format!("{CLIENT_KEY}:{}", user_uuid)
}
