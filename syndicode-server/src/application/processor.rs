use super::ports::processor::GameTickProcessable;

pub struct GameTickProcessor {}

impl GameTickProcessor {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl GameTickProcessable for GameTickProcessor {
    async fn process_next_tick(&self) -> anyhow::Result<usize> {
        // 1. Read Current State & Tick (N) from PG
        // 2. Pull Actions
        // tracing::debug!(num_actions = actions.len(), "Pulled actions.");

        // 3. Calculate State N+1
        tracing::debug!("Calculated next state.");

        // 4. Write State N+1 Atomically
        tracing::debug!("Atomically wrote next state.");

        // 5. Acknowledge processed actions
        // if !actions.is_empty() {
        //      let action_ids: Vec<&str> = actions.iter().map(|a| a.stream_id.as_str()).collect();
        //      self.action_puller
        //           .acknowledge_actions(&self.config.action_stream_key, &self.config.action_group_name, &action_ids)
        //           .await
        //           .context("Failed to acknowledge actions")?; // Decide if this error is critical
        //      tracing::debug!(num_acked = action_ids.len(), "Acknowledged processed actions.");
        // }
        //
        Ok(1)
    }
}
