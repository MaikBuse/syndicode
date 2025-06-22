use super::economy::building::PgBuildingRepository;
use super::economy::business::PgBusinessRepository;
use super::economy::business_listing::PgBusinessListingRepository;
use super::economy::business_offer::PgBusinessOfferRepository;
use super::economy::corporation::PgCorporationRepository;
use super::economy::market::PgMarketRepository;
use super::game_tick::PgGameTickRepository;
use super::init::PgInitializationRepository;
use super::unit::PgUnitRepository;
use super::user::PgUserRepository;
use super::user_verify::PgUserVerificationRepository;
use super::PostgresDatabase;
use crate::application::error::{ApplicationError, ApplicationResult};
use crate::application::ports::uow::{TransactionalContext, UnitOfWork};
use sqlx::{Postgres, Transaction};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct PgTransactionContext<'a, 'tx>
where
    'tx: 'a,
{
    pub tx: &'a mut Transaction<'tx, Postgres>,
    pub game_tick_repo: &'a PgGameTickRepository,
    pub init_repo: &'a PgInitializationRepository,
    pub user_repo: &'a PgUserRepository,
    pub user_verify_repo: &'a PgUserVerificationRepository,
    pub corporation_repo: &'a PgCorporationRepository,
    pub business_repo: &'a PgBusinessRepository,
    pub market_repo: &'a PgMarketRepository,
    pub unit_repo: &'a PgUnitRepository,
    pub business_listing_repo: &'a PgBusinessListingRepository,
    pub business_offer_repo: &'a PgBusinessOfferRepository,
    pub building_repo: &'a PgBuildingRepository,
}

// Implement the marker trait. Note the lifetimes match the struct.
// The 'a from the trait definition corresponds to the 'a lifetime here.
impl<'a> TransactionalContext<'a> for PgTransactionContext<'a, '_> {}

#[derive(Clone)]
pub struct PostgresUnitOfWork {
    pg_db: Arc<PostgresDatabase>,
    game_tick: PgGameTickRepository,
    init_repo: PgInitializationRepository,
    user_repo: PgUserRepository,
    user_verify_repo: PgUserVerificationRepository,
    corporation_repo: PgCorporationRepository,
    business_repo: PgBusinessRepository,
    market_repo: PgMarketRepository,
    unit_repo: PgUnitRepository,
    business_listing_repo: PgBusinessListingRepository,
    business_offer_repo: PgBusinessOfferRepository,
    building_repo: PgBuildingRepository,
}

impl PostgresUnitOfWork {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            game_tick: PgGameTickRepository,
            init_repo: PgInitializationRepository,
            user_repo: PgUserRepository,
            user_verify_repo: PgUserVerificationRepository,
            corporation_repo: PgCorporationRepository,
            business_repo: PgBusinessRepository,
            market_repo: PgMarketRepository,
            unit_repo: PgUnitRepository,
            business_listing_repo: PgBusinessListingRepository,
            business_offer_repo: PgBusinessOfferRepository,
            building_repo: PgBuildingRepository,
        }
    }
}

#[tonic::async_trait]
impl UnitOfWork for PostgresUnitOfWork {
    async fn execute<F, R>(&self, f: F) -> ApplicationResult<R>
    where
        // The 'a lifetime here applies to the borrow of the TransactionContext
        F: for<'a> FnOnce(
                &'a mut dyn TransactionalContext<'a>,
            )
                -> Pin<Box<dyn Future<Output = ApplicationResult<R>> + Send + 'a>>
            + Send,
        R: Send,
    {
        // 1. Begin Transaction - Keep it mutable and owned by this function
        let mut tx: Transaction<'static, Postgres> = self
            .pg_db
            .pool
            .begin()
            .await
            .map_err(ApplicationError::from)?;

        // 2. Introduce a scope for the context and its borrow of 'tx'
        let result: ApplicationResult<R> = {
            // Create context borrowing `tx` mutably. The lifetime 'a for the closure
            // will be inferred from the lifetime of this borrow.
            let mut context = PgTransactionContext {
                tx: &mut tx,
                game_tick_repo: &self.game_tick,
                init_repo: &self.init_repo,
                user_repo: &self.user_repo,
                user_verify_repo: &self.user_verify_repo,
                corporation_repo: &self.corporation_repo,
                business_repo: &self.business_repo,
                market_repo: &self.market_repo,
                unit_repo: &self.unit_repo,
                business_listing_repo: &self.business_listing_repo,
                business_offer_repo: &self.business_offer_repo,
                building_repo: &self.building_repo,
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
                    tracing::error!(
                        "Failed to rollback transaction after error: {:?}. Rollback error: {}",
                        error,
                        rollback_err
                    );
                }
                Err(error)
            }
        }
    }
}
