use super::uow::PgTransactionContext;
use crate::domain::{
    repository::RepositoryResult,
    user_verify::{
        model::{code::VerificationCode, UserVerification},
        repository::UserVerificationTxRepository,
    },
};
use sqlx::Postgres;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgUserVerificationRepository;

impl PgUserVerificationRepository {
    pub async fn create_user_verification(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user_verification: &UserVerification,
    ) -> RepositoryResult<()> {
        let user_uuid = user_verification.get_user_uuid();
        let code = user_verification.get_code();
        let expires_at = user_verification.get_expires_at();
        let created_at = user_verification.get_created_at();

        sqlx::query!(
            r#"
            INSERT INTO user_verifications (
                user_uuid,
                code,
                expires_at,
                created_at
            )
            VALUES ( $1, $2, $3, $4 )
            "#,
            user_uuid,
            code,
            expires_at,
            created_at
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn get_user_verification(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user_uuid: Uuid,
    ) -> RepositoryResult<UserVerification> {
        let record = sqlx::query!(
            r#"
            SELECT
                user_uuid,
                code,
                expires_at,
                created_at
            FROM user_verifications
            WHERE
                user_uuid = $1
            "#,
            user_uuid
        )
        .fetch_one(executor)
        .await?;

        let code = VerificationCode::from_input()
            .expires_at(record.expires_at)
            .created_at(record.created_at)
            .code(record.code)
            .call();

        let user_verification = UserVerification::builder()
            .user_uuid(user_uuid)
            .code(code)
            .build();

        Ok(user_verification)
    }

    pub async fn update_user_verification(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user_verification: &UserVerification,
    ) -> RepositoryResult<()> {
        let user_uuid = user_verification.get_user_uuid();
        let code = user_verification.get_code();
        let expires_at = user_verification.get_expires_at();
        let created_at = user_verification.get_created_at();

        sqlx::query!(
            r#"
            UPDATE user_verifications
            SET
                code=$2,
                expires_at=$3,
                created_at=$4
            WHERE
                user_uuid=$1
            "#,
            user_uuid,
            code,
            expires_at,
            created_at
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn delete_user_verification(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user_uuid: Uuid,
    ) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM user_verifications
            WHERE user_uuid = $1
            "#,
            user_uuid
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl UserVerificationTxRepository for PgTransactionContext<'_, '_> {
    async fn create_user_verification(
        &mut self,
        user_verification: &UserVerification,
    ) -> RepositoryResult<()> {
        self.user_verify_repo
            .create_user_verification(&mut **self.tx, user_verification)
            .await
    }

    async fn get_user_verification(
        &mut self,
        user_uuid: Uuid,
    ) -> RepositoryResult<UserVerification> {
        self.user_verify_repo
            .get_user_verification(&mut **self.tx, user_uuid)
            .await
    }

    async fn update_user_verification(
        &mut self,
        user_verification: &UserVerification,
    ) -> RepositoryResult<()> {
        self.user_verify_repo
            .update_user_verification(&mut **self.tx, user_verification)
            .await
    }

    async fn delete_user_verification(&mut self, user_uuid: Uuid) -> RepositoryResult<()> {
        self.user_verify_repo
            .delete_user_verification(&mut **self.tx, user_uuid)
            .await
    }
}
