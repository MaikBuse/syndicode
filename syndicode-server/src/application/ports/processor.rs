#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[tonic::async_trait]
pub trait GameTickProcessable: Send + Sync {
    async fn process_next_tick(&self) -> anyhow::Result<usize>;
}
