pub mod control;
pub mod economy;
pub mod warfare;

use argon2::Argon2;
use rand::TryRngCore;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;
use uuid::Uuid;

use crate::domain::{
    model::control::{UserModel, UserRole},
    repository::control::ControlDatabaseRepository,
};

#[derive(Debug)]
pub struct SqliteDatabase {
    pool: Pool<Sqlite>,
}

impl SqliteDatabase {
    pub async fn init(admin_password: String) -> anyhow::Result<SqliteDatabase> {
        let database_url =
            env::var("DATABASE_URL").expect("Environment variable 'DATABASE_URL' must be set");

        let pool = SqlitePool::connect(&database_url).await?;

        let user_uuid = Uuid::now_v7();
        let mut salt = [0u8; 16];
        if let Err(err) = rand::rng().try_fill_bytes(&mut salt) {
            return Err(anyhow::anyhow!("Failed to generate salt: {}", err).into());
        }

        let argon = Argon2::default();

        let mut password_hashed = Vec::<u8>::new();
        if let Err(err) =
            argon.hash_password_into(&admin_password.as_bytes(), &salt, &mut password_hashed)
        {
            return Err(anyhow::anyhow!("Failed to hash password: {}", err).into());
        }

        let user = UserModel {
            uuid: user_uuid.into(),
            name: "Admin".to_string(),
            password_hash: password_hashed,
            salt: salt.into(),
            role: UserRole::Admin.to_string(),
        };

        let sqlite_db = Self { pool };

        sqlite_db.create_user(user).await?;

        Ok(sqlite_db)
    }
}
