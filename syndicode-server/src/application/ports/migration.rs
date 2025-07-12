#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::domain::repository::RepositoryResult;

#[cfg_attr(test, automock)]
#[tonic::async_trait]
pub trait MigrationRunner {
    async fn run_migration(&self) -> RepositoryResult<()>;
}
