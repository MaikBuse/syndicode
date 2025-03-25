pub mod control;
pub mod economy;
pub mod warfare;

use crate::domain::{
    model::control::{UserModel, UserRole},
    repository::control::ControlDatabaseRepository,
};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHasher};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;
use uuid::Uuid;

const ADMIN_USERNAME: &str = "admin";

#[derive(Debug)]
pub struct SqliteDatabase {
    pool: Pool<Sqlite>,
}

impl SqliteDatabase {
    pub async fn init(admin_password: String) -> anyhow::Result<SqliteDatabase> {
        let database_url =
            env::var("DATABASE_URL").expect("Environment variable 'DATABASE_URL' must be set");

        let pool = SqlitePool::connect(&database_url).await?;

        let sqlite_db = Self { pool };

        if sqlite_db
            .get_user_by_name(ADMIN_USERNAME.to_string())
            .await
            .is_err()
        {
            let user_uuid = Uuid::now_v7();

            let argon2 = Argon2::default();
            let salt = SaltString::generate(&mut OsRng);

            let password_hash = match argon2.hash_password(admin_password.as_bytes(), &salt) {
                Ok(pw) => pw,
                Err(err) => {
                    return Err(anyhow::anyhow!(
                        "Failed to hash password: {}",
                        err.to_string()
                    ))
                }
            };

            let user = UserModel {
                uuid: user_uuid.into(),
                name: ADMIN_USERNAME.to_string(),
                password_hash: password_hash.to_string(),
                role: UserRole::Admin.to_string(),
            };

            sqlite_db.create_user(user).await?;
        }

        Ok(sqlite_db)
    }
}
