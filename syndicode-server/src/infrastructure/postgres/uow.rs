use super::corporation::PgCorporationRepository;
use super::unit::PgUnitRepository;
use super::user::PgUserRepository;
use crate::application::uow::{TransactionalContext, UnitOfWork};
use crate::domain::repository::RepositoryResult;
use sqlx::{PgPool, Postgres, Transaction};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct PgTransactionContext<'a, 'tx>
where
    'tx: 'a,
{
    pub tx: &'a mut Transaction<'tx, Postgres>,
    pub user_repo: &'a PgUserRepository,
    pub corporation_repo: &'a PgCorporationRepository,
    pub unit_repo: &'a PgUnitRepository,
}

// Implement the marker trait. Note the lifetimes match the struct.
// The 'a from the trait definition corresponds to the 'a lifetime here.
impl<'a, 'tx> TransactionalContext<'a> for PgTransactionContext<'a, 'tx> {}

#[derive(Clone)]
pub struct PostgresUnitOfWork {
    pool: Arc<PgPool>,
    user_repo: PgUserRepository,
    corporation_repo: PgCorporationRepository,
    unit_repo: PgUnitRepository,
}

impl PostgresUnitOfWork {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            user_repo: PgUserRepository,
            corporation_repo: PgCorporationRepository,
            unit_repo: PgUnitRepository,
        }
    }
}

#[tonic::async_trait]
impl UnitOfWork for PostgresUnitOfWork {
    async fn execute<F, R>(&self, f: F) -> RepositoryResult<R>
    where
        // The 'a lifetime here applies to the borrow of the TransactionContext
        F: for<'a> FnOnce(
                &'a mut dyn TransactionalContext<'a>,
            )
                -> Pin<Box<dyn Future<Output = RepositoryResult<R>> + Send + 'a>>
            + Send,
        R: Send,
    {
        // 1. Begin Transaction - Keep it mutable and owned by this function
        let mut tx: Transaction<'static, Postgres> = self.pool.begin().await?;

        // 2. Introduce a scope for the context and its borrow of 'tx'
        let result: RepositoryResult<R> = {
            // Create context borrowing `tx` mutably. The lifetime 'a for the closure
            // will be inferred from the lifetime of this borrow.
            let mut context = PgTransactionContext {
                tx: &mut tx,
                user_repo: &self.user_repo,
                corporation_repo: &self.corporation_repo,
                unit_repo: &self.unit_repo,
            };

            // Execute the closure, await the future INSIDE the scope.
            // The future borrows `context`, which borrows `tx`.
            f(&mut context).await
        }; // <-- Scope ends here. `context` is dropped. The mutable borrow of `tx` ends.

        // 3. `tx` is now owned solely by this function again. Commit or rollback.
        match result {
            Ok(value) => {
                // Commit the original transaction `tx`
                tx.commit().await?;
                Ok(value)
            }
            Err(error) => {
                // Rollback the original transaction `tx`
                if let Err(rollback_err) = tx.rollback().await {
                    eprintln!(
                        "Failed to rollback transaction after error: {:?}. Rollback error: {}",
                        error, rollback_err
                    );
                }
                Err(error)
            }
        }
    }
}
