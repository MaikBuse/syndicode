use crate::{
    application::ports::init::{InitializationRepository, InitializationTxRepository},
    domain::repository::RepositoryResult,
};
use sqlx::{PgPool, Postgres};
use std::sync::Arc;

use super::uow::PgTransactionContext;

const INIT_FLAG_KEY: &str = "database_initialized";

const INIT_ADVISORY_LOCK_KEY: i64 = 42; // unique number

#[derive(Clone)]
pub struct PgInitializationRepository;

impl PgInitializationRepository {
    pub async fn is_database_initialized(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
    ) -> RepositoryResult<bool> {
        let is_set: Option<bool> = sqlx::query_scalar!(
            "SELECT is_set FROM system_flags WHERE flag_key = $1",
            INIT_FLAG_KEY
        )
        .fetch_optional(executor)
        .await?;

        match is_set {
            Some(is_set) => Ok(is_set),
            None => Ok(false),
        }
    }

    pub async fn set_database_initialization_flag(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
    ) -> RepositoryResult<()> {
        sqlx::query!(
            "UPDATE system_flags SET is_set = TRUE, updated_at = NOW() WHERE flag_key = $1",
            INIT_FLAG_KEY
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn set_advisory_lock(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
    ) -> RepositoryResult<()> {
        sqlx::query!("SELECT pg_advisory_lock($1)", INIT_ADVISORY_LOCK_KEY)
            .execute(executor)
            .await?;

        Ok(())
    }
}

pub struct PgInitializationService {
    pool: Arc<PgPool>,
    init_repo: PgInitializationRepository,
}

impl PgInitializationService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            init_repo: PgInitializationRepository,
        }
    }
}

#[tonic::async_trait]
impl InitializationRepository for PgInitializationService {
    async fn is_database_initialized(&self) -> RepositoryResult<bool> {
        self.init_repo.is_database_initialized(&*self.pool).await
    }

    async fn set_advisory_lock(&self) -> RepositoryResult<()> {
        self.init_repo.set_advisory_lock(&*self.pool).await
    }
}

#[tonic::async_trait]
impl InitializationTxRepository for PgTransactionContext<'_, '_> {
    async fn is_database_initialized(&mut self) -> RepositoryResult<bool> {
        self.init_repo.is_database_initialized(&mut **self.tx).await
    }

    async fn set_database_initialization_flag(&mut self) -> RepositoryResult<()> {
        self.init_repo
            .set_database_initialization_flag(&mut **self.tx)
            .await
    }
}
