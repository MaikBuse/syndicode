use super::SqliteDatabase;
use crate::domain::{
    model::control::{SessionModel, SessionUser, UserModel},
    repository::control::{ControlDatabaseRepository, ControlDatabaseResult},
};
use tonic::async_trait;

#[async_trait]
impl ControlDatabaseRepository for SqliteDatabase {
    async fn create_user(&self, user: UserModel) -> ControlDatabaseResult<UserModel> {
        let user = sqlx::query_as!(
            UserModel,
            r#"
            INSERT INTO users (
                uuid,
                name,
                password_hash,
                salt,
                role
            )
            VALUES ( ?1, ?2, ?3, ?4, ?5 )
            RETURNING
                uuid,
                name,
                password_hash,
                salt,
                role
            "#,
            user.uuid,
            user.name,
            user.password_hash,
            user.salt,
            user.role
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user(&self, user_uuid: Vec<u8>) -> ControlDatabaseResult<UserModel> {
        let user = sqlx::query_as!(
            UserModel,
            r#"
            SELECT
                uuid,
                name,
                password_hash,
                salt,
                role
            FROM users
            WHERE
                uuid = ?1
            "#,
            user_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_name(&self, username: String) -> ControlDatabaseResult<UserModel> {
        let user = sqlx::query_as!(
            UserModel,
            r#"
            SELECT
                uuid,
                name,
                password_hash,
                salt,
                role
            FROM users
            WHERE
                name = ?1
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn delete_user(&self, user_uuid: Vec<u8>) -> ControlDatabaseResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE uuid = ?1
            "#,
            user_uuid
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn create_session(&self, session_uuid: Vec<u8>) -> ControlDatabaseResult<SessionModel> {
        let session = sqlx::query_as!(
            SessionModel,
            r#"
            INSERT INTO sessions (uuid)
            VALUES (?1)
            RETURNING uuid, interval, state
            "#,
            session_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    async fn get_session(&self, session_uuid: Vec<u8>) -> ControlDatabaseResult<SessionModel> {
        let session = sqlx::query_as!(
            SessionModel,
            r#"
            SELECT
                uuid,
                interval,
                state
            FROM sessions
            WHERE
                uuid = ?1
            "#,
            session_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    async fn list_sessions(&self) -> ControlDatabaseResult<Vec<SessionModel>> {
        let sessions = sqlx::query_as!(
            SessionModel,
            r#"
            SELECT
                uuid,
                interval,
                state
            FROM sessions
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(sessions)
    }

    async fn update_session(&self, session: SessionModel) -> ControlDatabaseResult<SessionModel> {
        let session = sqlx::query_as!(
            SessionModel,
            r#"
            UPDATE sessions
            SET
                interval = ?2,
                state = ?3
            WHERE uuid = ?1
            RETURNING uuid, interval, state
            "#,
            session.uuid,
            session.interval,
            session.state
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    async fn delete_session(&self, session_uuid: Vec<u8>) -> ControlDatabaseResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE uuid = ?1
            "#,
            session_uuid
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_session_user(
        &self,
        session_uuid: Vec<u8>,
        user_uuid: Vec<u8>,
    ) -> ControlDatabaseResult<SessionUser> {
        let session_user = sqlx::query_as!(
            SessionUser,
            r#"
            SELECT
                uuid,
                session_uuid,
                user_uuid
            FROM session_users
            WHERE
                session_uuid = ?1
                AND user_uuid = ?2
            "#,
            session_uuid,
            user_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(session_user)
    }

    async fn create_session_user(
        &self,
        session_user: SessionUser,
    ) -> ControlDatabaseResult<SessionUser> {
        let session_user = sqlx::query_as!(
            SessionUser,
            r#"
            INSERT INTO session_users (
                uuid,
                session_uuid,
                user_uuid
            )
            VALUES (?1, ?2, ?3)
            RETURNING uuid, session_uuid, user_uuid
            "#,
            session_user.uuid,
            session_user.session_uuid,
            session_user.user_uuid,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(session_user)
    }
}
