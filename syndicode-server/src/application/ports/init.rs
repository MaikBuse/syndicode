use std::fmt::Display;

use crate::domain::repository::RepositoryResult;

pub enum FlagKey {
    Database,
    AdminDomain,
    EconomyDomain,
}

impl Display for FlagKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlagKey::Database => write!(f, "database_initialized"),
            FlagKey::AdminDomain => write!(f, "admin_domain_initialized"),
            FlagKey::EconomyDomain => write!(f, "economy_domain_initialized"),
        }
    }
}

#[tonic::async_trait]
pub trait InitializationRepository: Send + Sync {
    async fn is_flag_set(&self, flag: FlagKey) -> RepositoryResult<bool>;
    async fn set_flag(&self, flag: FlagKey) -> RepositoryResult<()>;
    async fn set_advisory_lock(&self) -> RepositoryResult<()>;
}

#[tonic::async_trait]
pub trait InitializationTxRepository: Send + Sync {
    async fn is_flag_set(&mut self, flag: FlagKey) -> RepositoryResult<bool>;
    async fn set_flag(&mut self, flag: FlagKey) -> RepositoryResult<()>;
}
