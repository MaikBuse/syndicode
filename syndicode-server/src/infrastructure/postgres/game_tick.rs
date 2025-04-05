use crate::domain::repository::RepositoryResult;
use sqlx::Postgres;

#[derive(Clone)]
pub struct PgGameTickRepository;

impl PgGameTickRepository {
    /// Retrieves the current game tick number from the dedicated table.
    /// This is essential before querying the state tables for the "live" state.
    pub async fn get_current_game_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
    ) -> RepositoryResult<i64> {
        let record = sqlx::query!(
            r#"
            SELECT current_game_tick FROM current_game_tick WHERE singleton_key = TRUE
            "#
        )
        .fetch_one(executor)
        .await?;

        Ok(record.current_game_tick)
    }

    /// Updates the current game tick pointer.
    /// This should ONLY be called by the Leader process within the atomic transaction
    /// that also inserts the new state rows.
    pub async fn update_current_game_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        new_game_tick: i64,
    ) -> RepositoryResult<()> {
        let rows_affected = sqlx::query!(
            r#"
             UPDATE current_game_tick
             SET current_game_tick = $1
             WHERE singleton_key = TRUE
             "#,
            new_game_tick
        )
        .execute(executor)
        .await?
        .rows_affected();

        if rows_affected == 1 {
            Ok(())
        } else {
            // This indicates a serious problem, as the singleton row should always exist
            Err(sqlx::Error::RowNotFound.into())
        }
    }
}
