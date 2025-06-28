use crate::{
    application::ports::init::{FlagKey, InitializationRepository, InitializationTxRepository},
    domain::repository::RepositoryResult,
};
use sqlx::Postgres;
use std::sync::Arc;

use super::{uow::PgTransactionContext, PostgresDatabase};

const INIT_ADVISORY_LOCK_KEY: i64 = 42; // unique number

#[derive(Clone)]
pub struct PgInitializationRepository;

impl PgInitializationRepository {
    pub async fn is_flag_set(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        flag: FlagKey,
    ) -> RepositoryResult<bool> {
        let is_set: Option<bool> = sqlx::query_scalar!(
            "SELECT is_set FROM system_flags WHERE flag_key = $1",
            flag.to_string()
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
        flag: FlagKey,
    ) -> RepositoryResult<()> {
        sqlx::query!(
            "UPDATE system_flags SET is_set = TRUE, updated_at = NOW() WHERE flag_key = $1",
            flag.to_string()
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
    pg_db: Arc<PostgresDatabase>,
    init_repo: PgInitializationRepository,
}

impl PgInitializationService {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            init_repo: PgInitializationRepository,
        }
    }
}

#[tonic::async_trait]
impl InitializationRepository for PgInitializationService {
    async fn is_flag_set(&self, flag: FlagKey) -> RepositoryResult<bool> {
        self.init_repo.is_flag_set(&self.pg_db.pool, flag).await
    }

    async fn set_flag(&self, flag: FlagKey) -> RepositoryResult<()> {
        self.init_repo
            .set_database_initialization_flag(&self.pg_db.pool, flag)
            .await
    }

    async fn set_advisory_lock(&self) -> RepositoryResult<()> {
        self.init_repo.set_advisory_lock(&self.pg_db.pool).await
    }
}

#[tonic::async_trait]
impl InitializationTxRepository for PgTransactionContext<'_, '_> {
    async fn is_flag_set(&mut self, flag: FlagKey) -> RepositoryResult<bool> {
        self.init_repo.is_flag_set(&mut **self.tx, flag).await
    }

    async fn set_flag(&mut self, flag: FlagKey) -> RepositoryResult<()> {
        self.init_repo
            .set_database_initialization_flag(&mut **self.tx, flag)
            .await
    }
}
