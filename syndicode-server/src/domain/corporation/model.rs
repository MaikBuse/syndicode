pub mod name;

use name::CorporationName;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

const DEFAULT_BALANCE: i64 = 1000000;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Corporation {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub name: CorporationName,
    pub balance: i64,
}

impl Corporation {
    pub fn new(user_uuid: Uuid, name: CorporationName) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            user_uuid,
            name,
            balance: DEFAULT_BALANCE,
        }
    }
}
