use super::{uow::PgTransactionContext, PostgresDatabase};
use crate::domain::{
    repository::{RepositoryError, RepositoryResult},
    user::{
        model::User,
        repository::{UserRepository, UserTxRepository},
    },
};
use sqlx::Postgres;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgUserRepository;

impl PgUserRepository {
    pub async fn create_user(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user: &User,
    ) -> RepositoryResult<()> {
        let user_role: i16 = user.role.into();
        let user_name = user.name.clone().into_inner();
        let user_email = user.email.clone().into_inner();
        let user_status = user.status.to_string();

        if let Err(err) = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (
                uuid,
                name,
                password_hash,
                email,
                role,
                status
            )
            VALUES ( $1, $2, $3, $4, $5, $6 )
            "#,
            user.uuid,
            user_name,
            user.password_hash,
            user_email,
            user_role,
            user_status
        )
        .execute(executor)
        .await
        {
            if let sqlx::Error::Database(database_error) = &err {
                if let Some(c) = database_error.constraint() {
                    match c {
                        "users_name_key" => return Err(RepositoryError::UserNameAlreadyTaken),
                        "users_email_key" => return Err(RepositoryError::EmailInUse),
                        _ => {
                            return Err(anyhow::anyhow!("{}", database_error.to_string()).into());
                        }
                    }
                }
            }

            return Err(err.into());
        }

        Ok(())
    }

    pub async fn get_user(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user_uuid: Uuid,
    ) -> RepositoryResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                uuid,
                name,
                password_hash,
                email,
                role,
                status
            FROM users
            WHERE
                uuid = $1
            "#,
            user_uuid
        )
        .fetch_one(executor)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_name(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user_name: String,
    ) -> RepositoryResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                uuid,
                name,
                password_hash,
                email,
                role,
                status
            FROM users
            WHERE
                name = $1
            "#,
            user_name
        )
        .fetch_one(executor)
        .await?;

        Ok(user)
    }

    pub async fn update_user(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user: &User,
    ) -> RepositoryResult<()> {
        let user_role: i16 = user.role.into();
        let user_name = user.name.clone().into_inner();
        let user_email = user.email.clone().into_inner();
        let user_status = user.status.to_string();

        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                name=$2,
                password_hash=$3,
                email=$4,
                role=$5,
                status=$6
            WHERE
                uuid=$1
            "#,
            user.uuid,
            user_name,
            user.password_hash,
            user_email,
            user_role,
            user_status
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn delete_user(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user_uuid: Uuid,
    ) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE uuid = $1
            "#,
            user_uuid
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}

pub struct PgUserService {
    pg_db: Arc<PostgresDatabase>,
    user_repo: PgUserRepository,
}

impl PgUserService {
    pub fn new(pg_db: Arc<PostgresDatabase>, user_repo: PgUserRepository) -> Self {
        Self { pg_db, user_repo }
    }
}

#[tonic::async_trait]
impl UserRepository for PgUserService {
    async fn create_user(&self, user: &User) -> RepositoryResult<()> {
        self.user_repo.create_user(&self.pg_db.pool, user).await
    }

    async fn get_user(&self, user_uuid: Uuid) -> RepositoryResult<User> {
        self.user_repo.get_user(&self.pg_db.pool, user_uuid).await
    }

    async fn get_user_by_name(&self, user_name: String) -> RepositoryResult<User> {
        self.user_repo
            .get_user_by_name(&self.pg_db.pool, user_name)
            .await
    }

    async fn delete_user(&self, user_uuid: Uuid) -> RepositoryResult<()> {
        self.user_repo
            .delete_user(&self.pg_db.pool, user_uuid)
            .await
    }
}

#[tonic::async_trait]
impl UserTxRepository for PgTransactionContext<'_, '_> {
    async fn create_user(&mut self, user: &User) -> RepositoryResult<()> {
        self.user_repo.create_user(&mut **self.tx, user).await
    }

    async fn get_user_by_name(&mut self, user_name: String) -> RepositoryResult<User> {
        self.user_repo
            .get_user_by_name(&mut **self.tx, user_name)
            .await
    }

    async fn update_user(&mut self, user: &User) -> RepositoryResult<()> {
        self.user_repo.update_user(&mut **self.tx, user).await
    }
}
