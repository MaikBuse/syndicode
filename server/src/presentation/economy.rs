use super::proto::{
    control::{game_update::ResponseEnum, GameUpdate},
    economy::{CorporationInfo, GetCorporationRequest, GetCorporationResponse},
};
use crate::service::economy::EconomyService;
use std::sync::Arc;
use tonic::{Code, Status};

pub async fn get_corporation(
    request: GetCorporationRequest,
    economy_service: Arc<EconomyService>,
    user_uuid: Vec<u8>,
) -> Result<GameUpdate, Status> {
    match economy_service
        .get_corporation(request.session_uuid, user_uuid)
        .await
    {
        Ok(corporation) => Ok(GameUpdate {
            response_enum: Some(ResponseEnum::GetCorporation(GetCorporationResponse {
                corporation: Some(CorporationInfo {
                    uuid: corporation.uuid,
                    session_uuid: corporation.session_uuid,
                    user_uuid: corporation.user_uuid,
                    name: corporation.name,
                    balance: corporation.balance,
                }),
            })),
        }),
        Err(err) => Err(Status::new(Code::Internal, err.to_string())),
    }
}
