use std::fmt::Display;

pub type RepositoryResult<T> = std::result::Result<T, RepositoryError>;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(i16)]
pub enum DomainSortDirection {
    #[default]
    Ascending,
    Descending,
}

impl Display for DomainSortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainSortDirection::Ascending => write!(f, "ASC"),
            DomainSortDirection::Descending => write!(f, "DESC"),
        }
    }
}

impl From<i16> for DomainSortDirection {
    fn from(value: i16) -> Self {
        match value {
            1 => Self::Ascending,
            _ => Self::Descending,
        }
    }
}

impl From<DomainSortDirection> for i16 {
    fn from(value: DomainSortDirection) -> Self {
        match value {
            DomainSortDirection::Ascending => 1,
            DomainSortDirection::Descending => 2,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("The provided user name has already been taken")]
    UserNameAlreadyTaken,

    #[error("The provided email is already in use")]
    EmailInUse,

    #[error("The database failed to return a result for the provided query")]
    NotFound,

    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
