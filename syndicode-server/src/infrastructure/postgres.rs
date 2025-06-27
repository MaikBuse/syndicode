pub mod economy;
pub mod game_tick;
pub mod init;
pub mod migration;
pub mod unit;
pub mod uow;
pub mod user;
pub mod user_verify;

use std::sync::Arc;

use crate::config::ServerConfig;
use geo::{LineString, Point, Polygon};
use sqlx::{
    pool::PoolOptions,
    postgres::types::{PgPoint, PgPolygon},
    PgPool,
};

#[derive(Debug)]
pub struct PostgresDatabase {
    pub pool: PgPool,
}

impl PostgresDatabase {
    pub async fn new(config: Arc<ServerConfig>) -> anyhow::Result<Self> {
        tracing::info!("Initializing postgres database connection");

        let conn_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            urlencoding::encode(config.postgres.user.as_str()),
            urlencoding::encode(config.postgres.password.as_str()),
            config.postgres.host,
            config.postgres.port,
            config.postgres.database
        );

        let pool = PoolOptions::new()
            .max_connections(config.postgres.max_connections)
            .connect(&conn_string)
            .await
            .map_err(|err| anyhow::format_err!(err))?;

        Ok(Self { pool })
    }
}

pub(super) fn from_geo_point_to_pg_point(point: Point) -> PgPoint {
    let (x, y) = point.x_y();

    PgPoint { x, y }
}

pub(super) fn from_geo_polygon_to_pg_points(polygon: Polygon<f64>) -> PgPolygon {
    let points: Vec<PgPoint> = polygon
        .exterior()
        .points()
        .map(|point| {
            let (x, y) = point.x_y();
            PgPoint { x, y }
        })
        .collect();

    PgPolygon { points }
}

#[allow(dead_code)]
pub(super) fn from_pg_point_to_geo_point(pg_point: PgPoint) -> Point<f64> {
    Point::new(pg_point.x, pg_point.y)
}

#[allow(dead_code)]
pub(super) fn from_pg_polygon_to_geo_polygon(pg_polygon: PgPolygon) -> Polygon<f64> {
    let exterior: Vec<_> = pg_polygon.points.iter().map(|p| (p.x, p.y)).collect();

    Polygon::new(LineString::from(exterior), vec![])
}
