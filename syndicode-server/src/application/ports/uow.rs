#[cfg(test)]
use mockall::{automock, predicate::*};

use super::game_tick::GameTickTxRepository;
use crate::{
    application::error::ApplicationResult,
    domain::{
        corporation::repository::CorporationTxRepository, unit::repository::UnitTxRespository,
        user::repository::UserTxRepository, user_verify::repository::UserVerificationTxRepository,
    },
};
use std::{future::Future, pin::Pin};
use tonic::async_trait;

// This trait combines all repositories needed within a single transaction.
// It acts as the handle passed to the business logic closure.
// The 'a lifetime ensures it cannot outlive the transaction scope.
pub trait TransactionalContext<'a>:
    GameTickTxRepository
    + UserTxRepository
    + UserVerificationTxRepository
    + CorporationTxRepository
    + UnitTxRespository
    + Send
    + Sync
{
    // This trait is just a marker/combiner, no methods needed here.
    // Add other repositories here as needed: + ProductRepository, etc.
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UnitOfWork: Send + Sync {
    /// Executes a closure within a database transaction.
    /// The closure receives a mutable reference to a TransactionalContext,
    /// which provides access to repositories operating within that transaction.
    async fn execute<F, R>(&self, f: F) -> ApplicationResult<R>
    where
        // F is a closure that takes the transactional context...
        F: for<'a> FnOnce(
                &'a mut dyn TransactionalContext<'a>,
            )
                -> Pin<Box<dyn Future<Output = ApplicationResult<R>> + Send + 'a>>
            + Send
            + 'static,
        // R is the successful return type of the closure's Future.
        R: Send + 'static;
}
