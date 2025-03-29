use super::common::parse_uuid;
use crate::service::economy::EconomyService;
use std::sync::Arc;
use syndicode_proto::{
    control::{game_update::ResponseEnum, GameUpdate},
    economy::{CorporationInfo, GetCorporationRequest, GetCorporationResponse},
};
use tonic::{Code, Status};
use uuid::Uuid;

pub async fn get_corporation(
    request: GetCorporationRequest,
    economy_service: Arc<EconomyService>,
    user_uuid: Uuid,
) -> Result<GameUpdate, Status> {
    let session_uuid = parse_uuid(&request.session_uuid)?;

    match economy_service
        .get_corporation(session_uuid, user_uuid)
        .await
    {
        Ok(corporation) => Ok(GameUpdate {
            response_enum: Some(ResponseEnum::GetCorporation(GetCorporationResponse {
                corporation: Some(CorporationInfo {
                    uuid: corporation.uuid.to_string(),
                    session_uuid: corporation.session_uuid.to_string(),
                    user_uuid: corporation.user_uuid.to_string(),
                    name: corporation.name,
                    balance: corporation.balance,
                }),
            })),
        }),
        Err(err) => Err(Status::new(Code::Internal, err.to_string())),
    }
}
