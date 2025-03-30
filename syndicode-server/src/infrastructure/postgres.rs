pub mod control;
pub mod economy;
pub mod warfare;

use crate::domain::model::control::{UserModel, UserRole};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHasher};
use sqlx::{pool::PoolOptions, PgPool, Pool, Postgres};
use std::env;
use uuid::Uuid;

pub const ADMIN_USERNAME: &str = "admin";

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),

    #[error("The database returned with a violation of a unique/primary key constraint")]
    UniqueConstraint,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type DatabaseResult<T> = std::result::Result<T, DatabaseError>;

#[derive(Debug)]
pub struct PostgresDatabase {
    pub pool: PgPool,
}

impl PostgresDatabase {
    pub async fn init(admin_password: String) -> anyhow::Result<Self> {
        tracing::info!("Initializing postgres database connection");

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
            urlencoding::encode(postgres_user.as_str()),
            urlencoding::encode(postgres_password.as_str()),
            postgres_host,
            postgres_port,
            postgres_db
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

        if let Err(err) = postgres_db.reate_user(&postgres_db.pool, user).await {
            match err {
                DatabaseError::UniqueConstraint => {
                    tracing::info!("Default admin user has already been created");
                }
                _ => return Err(err.into()),
            }
        }

        Ok(postgres_db)
    }

    pub async fn reate_user<'e, E>(&self, executor: E, user: UserModel) -> DatabaseResult<UserModel>
    where
        E: sqlx::Executor<'e, Database = Postgres> + Send,
    {
        let user_role: i16 = user.role.into();

        match sqlx::query_as!(
            UserModel,
            r#"
            INSERT INTO users (
                uuid,
                name,
                password_hash,
                role
            )
            VALUES ( $1, $2, $3, $4 )
            RETURNING
                uuid,
                name,
                password_hash,
                role
            "#,
            user.uuid,
            user.name,
            user.password_hash,
            user_role
        )
        .fetch_one(executor)
        .await
        {
            Ok(user) => Ok(user),
            Err(err) => match err {
                sqlx::Error::Database(database_error) => {
                    match database_error.is_unique_violation() {
                        true => Err(DatabaseError::UniqueConstraint),
                        false => Err(anyhow::anyhow!("{}", database_error.to_string()).into()),
                    }
                }
                _ => Err(err.into()),
            },
        }
    }
}
