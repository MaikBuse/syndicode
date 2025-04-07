use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

const DEFAULT_BALANCE: i64 = 1000000;

#[derive(Serialize, Deserialize, Debug, Clone, Validate, FromRow)]
pub struct Corporation {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    #[validate(length(min = 1, max = 20))]
    pub name: String,
    pub balance: i64,
}

impl Corporation {
    pub fn new(user_uuid: Uuid, name: String) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            user_uuid,
            name,
            balance: DEFAULT_BALANCE,
        }
    }
}
