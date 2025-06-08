use std::fmt::Display;
pub type RepositoryResult<T> = std::result::Result<T, RepositoryError>;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(i16)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

impl Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortDirection::Ascending => write!(f, "ASC"),
            SortDirection::Descending => write!(f, "DESC"),
        }
    }
}

impl From<i16> for SortDirection {
    fn from(value: i16) -> Self {
        match value {
            1 => Self::Ascending,
            _ => Self::Descending,
        }
    }
}

impl From<SortDirection> for i16 {
    fn from(value: SortDirection) -> Self {
        match value {
            SortDirection::Ascending => 1,
            SortDirection::Descending => 2,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),

    #[error("The database returned with a violation of a unique/primary key constraint")]
    UniqueConstraint,

    #[error("The database failed to return a result for the provided query")]
    NotFound,

    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
