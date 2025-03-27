use super::{
    common::parse_uuid,
    proto::{
        control::{game_update::ResponseEnum, GameUpdate},
        warfare::{
            ListUnitsRequest, ListUnitsResponse, SpawnUnitRequest, SpawnUnitResponse, UnitInfo,
        },
    },
};
use crate::{engine::Job, service::warfare::WarfareService};
use dashmap::DashMap;
use std::{collections::VecDeque, sync::Arc};
use tonic::{Code, Result, Status};
use uuid::Uuid;

pub async fn spawn_unit(
    request: SpawnUnitRequest,
    jobs: Arc<DashMap<Uuid, VecDeque<Job>>>,
) -> Result<GameUpdate, Status> {
    let session_uuid = parse_uuid(&request.session_uuid)?;
    let user_uuid = parse_uuid(&request.user_uuid)?;

    let mut session_jobs = jobs.entry(session_uuid).or_default();

    session_jobs.push_front(Job::UnitSpawn { user_uuid });

    Ok(GameUpdate {
        response_enum: Some(ResponseEnum::SpawnUnit(SpawnUnitResponse {})),
    })
}

pub async fn list_units(
    request: ListUnitsRequest,
    warfare_service: Arc<WarfareService>,
    user_uuid: Uuid,
) -> Result<GameUpdate, Status> {
    let session_uuid = parse_uuid(&request.session_uuid)?;

    let units = match warfare_service.list_units(session_uuid, user_uuid).await {
        Ok(units) => units,
        Err(err) => return Err(Status::new(Code::Internal, err.to_string())),
    };

    let mut unit_infos = Vec::<UnitInfo>::with_capacity(units.len());
    for u in units {
        unit_infos.push(UnitInfo {
            uuid: u.uuid.to_string(),
            session_uuid: u.session_uuid.to_string(),
            user_uuid: u.user_uuid.to_string(),
        });
    }

    Ok(GameUpdate {
        response_enum: Some(ResponseEnum::ListUnits(ListUnitsResponse {
            units: unit_infos,
        })),
    })
}
