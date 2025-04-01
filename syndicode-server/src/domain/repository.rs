pub mod corporation;
pub mod unit;
pub mod user;

pub type RepositoryResult<T> = std::result::Result<T, RepositoryError>;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),

    #[error("The database returned with a violation of a unique/primary key constraint")]
    UniqueConstraint,

    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
