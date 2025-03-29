pub mod control;
pub mod economy;
pub mod warfare;

use crate::domain::{
    model::control::{UserModel, UserRole},
    repository::control::{ControlDatabaseError, ControlDatabaseRepository},
};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHasher};
use sqlx::{pool::PoolOptions, PgPool, Pool, Postgres};
use std::env;
use uuid::Uuid;

pub const ADMIN_USERNAME: &str = "admin";

#[derive(Debug)]
pub struct PostgresDatabase {
    pool: PgPool,
}

impl PostgresDatabase {
    pub async fn init(admin_password: String) -> anyhow::Result<Self> {
        let postgres_user =
            env::var("POSTGRES_USER").expect("Environment variable 'POSTGRES_USER' must be set");
        let postgres_password = env::var("POSTGRES_PASSWORD")
            .expect("Environment variable 'POSTGRES_PASSWORD' must be set");
        let postgres_host =
            env::var("POSTGRES_HOST").expect("Environment variable 'POSTGRES_HOST' must be set");
        let postgres_port =
            env::var("POSTGRES_PORT").expect("Environment variable 'POSTGRES_PORT' must be set");
        let postgres_db =
            env::var("POSTGRES_DB").expect("Environment variable 'POSTGRES_DB' must be set");

        let conn_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            postgres_user, postgres_password, postgres_host, postgres_port, postgres_db
        );

        let pool: Pool<Postgres> = PoolOptions::new()
            .max_connections(5)
            .connect(&conn_string)
            .await?;

        sqlx::migrate!().run(&pool).await?;

        let postgres_db = Self { pool };

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
            uuid: user_uuid,
            name: ADMIN_USERNAME.to_string(),
            password_hash: password_hash.to_string(),
            role: UserRole::Admin,
        };

        if let Err(err) = postgres_db.create_user(user).await {
            match err {
                ControlDatabaseError::UniqueConstraint => {
                    tracing::info!("Default admin user has already been created");
                }
                _ => return Err(err.into()),
            }
        }

        Ok(postgres_db)
    }
}
