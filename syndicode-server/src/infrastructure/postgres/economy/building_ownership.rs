use std::sync::Arc;

use crate::{
    domain::{
        economy::building_ownership::{
            model::BuildingOwnership,
            repository::{BuildingOwnershipRepository, BuildingOwnershipTxRepository},
        },
        repository::RepositoryResult,
    },
    infrastructure::postgres::{uow::PgTransactionContext, PostgresDatabase},
};
use sqlx::Postgres;

#[derive(Clone)]
pub struct PgBuildingOwnershipRepository;

impl PgBuildingOwnershipRepository {
    /// This leverages PostgreSQL's UNNEST function for efficiency.
    /// CARE: This is not compile time checked
    pub async fn insert_building_ownerships_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        building_ownerships: &[BuildingOwnership],
        game_tick: i64,
    ) -> RepositoryResult<()> {
        if building_ownerships.is_empty() {
            return Ok(());
        }

        let count = building_ownerships.len();

        let mut building_uuid_vec = Vec::with_capacity(count);
        let mut owning_business_uuid_vec = Vec::with_capacity(count);

        for building_ownership in building_ownerships {
            building_uuid_vec.push(building_ownership.building_uuid);
            owning_business_uuid_vec.push(building_ownership.owning_business_uuid);
        }

        sqlx::query(
            r#"
            INSERT INTO building_ownerships (
                game_tick,
                building_uuid,
                owning_business_uuid
            )
            SELECT $1, u.*
            FROM unnest(
                $2::UUID[],
                $3::UUID[]
            )
            AS u(
                building_uuid,
                owning_business_uuid
            )
            "#,
        )
        .bind(game_tick)
        .bind(&building_uuid_vec)
        .bind(&owning_business_uuid_vec)
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn list_building_ownerships_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<Vec<BuildingOwnership>> {
        let records = sqlx::query!(
            r#"
            SELECT
                game_tick,
                building_uuid,
                owning_business_uuid
            FROM building_ownerships
            WHERE
                game_tick = $1
            "#,
            game_tick
        )
        .fetch_all(executor)
        .await?;

        let mut building_ownerships = Vec::with_capacity(records.len());
        for record in records {
            building_ownerships.push(BuildingOwnership {
                building_uuid: record.building_uuid,
                owning_business_uuid: record.owning_business_uuid,
            });
        }

        Ok(building_ownerships)
    }

    pub async fn delete_building_ownerships_before_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM building_ownerships
            WHERE
                game_tick < $1
            "#,
            game_tick
        )
        .execute(executor)
        .await?;

        Ok(result.rows_affected())
    }
}

pub struct PgBuildingOwnershipService {
    pg_db: Arc<PostgresDatabase>,
    building_ownership_repo: PgBuildingOwnershipRepository,
}

impl PgBuildingOwnershipService {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            building_ownership_repo: PgBuildingOwnershipRepository,
        }
    }
}

#[tonic::async_trait]
impl BuildingOwnershipRepository for PgBuildingOwnershipService {
    async fn list_building_ownerships_in_tick(
        &self,
        game_tick: i64,
    ) -> RepositoryResult<Vec<BuildingOwnership>> {
        self.building_ownership_repo
            .list_building_ownerships_in_tick(&self.pg_db.pool, game_tick)
            .await
    }
}

#[tonic::async_trait]
impl BuildingOwnershipTxRepository for PgTransactionContext<'_, '_> {
    async fn insert_building_ownerships_in_tick(
        &mut self,
        game_tick: i64,
        building_ownerships: &[BuildingOwnership],
    ) -> RepositoryResult<()> {
        self.building_ownerships_repo
            .insert_building_ownerships_in_tick(&mut **self.tx, building_ownerships, game_tick)
            .await
    }

    async fn delete_building_ownerships_before_tick(
        &mut self,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        self.building_ownerships_repo
            .delete_building_ownerships_before_tick(&mut **self.tx, game_tick)
            .await
    }
}
