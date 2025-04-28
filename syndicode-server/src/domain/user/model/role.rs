use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(i16)]
pub enum UserRole {
    Admin,
    #[default]
    Player,
}

impl From<i16> for UserRole {
    fn from(value: i16) -> Self {
        match value {
            1 => Self::Admin,
            _ => Self::Player,
        }
    }
}

impl From<UserRole> for i16 {
    fn from(val: UserRole) -> Self {
        match val {
            UserRole::Admin => 1,
            UserRole::Player => 2,
        }
    }
}

impl TryFrom<String> for UserRole {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Admin" => Ok(Self::Admin),
            "User" => Ok(Self::Player),
            _ => Err(anyhow::anyhow!(
                "Failed to parse user role '{}' from string",
                value
            )),
        }
    }
}

impl From<UserRole> for i32 {
    fn from(val: UserRole) -> Self {
        match val {
            UserRole::Admin => 1,
            UserRole::Player => 2,
        }
    }
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::Player => write!(f, "User"),
        }
    }
}
