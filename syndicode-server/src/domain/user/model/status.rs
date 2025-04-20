use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum UserStatus {
    Pending,
    Active,
    Suspended,
}

impl Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Pending => write!(f, "Pending"),
            UserStatus::Active => write!(f, "Active"),
            UserStatus::Suspended => write!(f, "Suspended"),
        }
    }
}

impl From<String> for UserStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Pending" => Self::Pending,
            "Active" => Self::Active,
            "Suspended" => Self::Suspended,
            _ => panic!("Failed to match '{}' to UserStatus", value),
        }
    }
}
