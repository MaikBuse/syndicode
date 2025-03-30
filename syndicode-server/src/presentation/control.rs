use super::common::parse_uuid;
use super::economy::get_corporation;
use super::middleware::USER_UUID_KEY;
use super::warfare::{list_units, spawn_unit};
use crate::domain::model::control::UserRole;
use crate::engine::Job;
use crate::service::control::ControlService;
use crate::service::economy::EconomyService;
use crate::service::error::ServiceError;
use crate::service::warfare::WarfareService;
use dashmap::DashMap;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use syndicode_proto::control::control_server::Control;
use syndicode_proto::control::game_update::ResponseEnum;
use syndicode_proto::control::user_request::RequestEnum;
use syndicode_proto::control::{
    CreateUserRequest, DeleteUserRequest, DeleteUserResponse, GameUpdate, RegistrationRequest,
    RegistrationResponse, UserRequest,
};
use syndicode_proto::control::{CreateUserResponse, UserRole as ProtoUserRole};
use syndicode_proto::control::{LoginRequest, LoginResponse};
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;
use tokio_stream::StreamExt;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::async_trait;
use tonic::metadata::MetadataMap;
use tonic::Streaming;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

type PlayerTx = mpsc::Sender<Result<GameUpdate, Status>>;

pub struct ControlPresenter {
    pub jobs: Arc<Mutex<VecDeque<Job>>>,
    pub user_channels: Arc<DashMap<Uuid, PlayerTx>>,
    pub control_service: Arc<ControlService>,
    pub economy_service: Arc<EconomyService>,
    pub warfare_service: Arc<WarfareService>,
}

#[async_trait]
impl Control for ControlPresenter {
    type GameStreamRpcStream = Pin<Box<dyn Stream<Item = Result<GameUpdate, Status>> + Send>>;

    async fn register(
        &self,
        request: Request<RegistrationRequest>,
    ) -> Result<Response<RegistrationResponse>, Status> {
        let req_user_uuid = match uuid_from_metadata(request.metadata()) {
            Ok(uuid) => uuid,
            Err(status) => return Err(status),
        };

        let request = request.into_inner();

        match self
            .control_service
            .create_user(
                req_user_uuid,
                request.username,
                request.password,
                UserRole::User,
            )
            .await
        {
            Ok(user) => Ok(Response::new(RegistrationResponse {
                uuid: user.uuid.to_string(),
            })),
            Err(err) => Err(control_error_into_status(err)),
        }
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let request = request.into_inner();

        let jwt = match self
            .control_service
            .login(request.username, request.password)
            .await
        {
            Ok(user) => user,
            Err(err) => {
                return Err(control_error_into_status(err));
            }
        };

        Ok(Response::new(LoginResponse { jwt }))
    }

    async fn game_stream_rpc(
        &self,
        request: Request<Streaming<UserRequest>>,
    ) -> Result<Response<Self::GameStreamRpcStream>, tonic::Status> {
        let req_user_uuid = match uuid_from_metadata(request.metadata()) {
            Ok(uuid) => uuid,
            Err(status) => return Err(status),
        };

        let mut stream = request.into_inner();

        // Channel to send game updates to this player
        let (tx, rx) = mpsc::channel(16);
        let player_rx = ReceiverStream::new(rx);

        self.user_channels.insert(req_user_uuid, tx.clone());

        // Use Arc for shared ownership
        let tx_arc = Arc::new(tx);
        let jobs = Arc::clone(&self.jobs);
        let control_service = Arc::clone(&self.control_service);
        let economy_service = Arc::clone(&self.economy_service);
        let warfare_service = Arc::clone(&self.warfare_service);

        // Spawn receiver of user actions
        tokio::spawn(async move {
            let tx = tx_arc.clone(); // Clone Arc to move into async block

            while let Some(Ok(action)) = stream.next().await {
                if let Some(request_enum) = action.request_enum {
                    match request_enum {
                        RequestEnum::CreateUser(req) => {
                            handle_request(
                                || create_user(req, Arc::clone(&control_service), req_user_uuid),
                                &tx,
                            )
                            .await;
                        }
                        RequestEnum::DeleteUser(req) => {
                            handle_request(|| delete_user(req, Arc::clone(&control_service)), &tx)
                                .await;
                        }
                        RequestEnum::GetCorporation(req) => {
                            handle_request(
                                || {
                                    get_corporation(
                                        req,
                                        Arc::clone(&economy_service),
                                        req_user_uuid,
                                    )
                                },
                                &tx,
                            )
                            .await;
                        }
                        RequestEnum::SpawnUnit(req) => {
                            handle_request(|| spawn_unit(req, Arc::clone(&jobs)), &tx).await;
                        }
                        RequestEnum::ListUnit(req) => {
                            handle_request(
                                || list_units(req, Arc::clone(&warfare_service), req_user_uuid),
                                &tx,
                            )
                            .await;
                        }
                    }
                }
            }
        });

        Ok(Response::new(
            Box::pin(player_rx) as Self::GameStreamRpcStream
        ))
    }
}

async fn handle_request<F, Fut>(fut: F, tx: &Arc<Sender<Result<GameUpdate, Status>>>)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<GameUpdate, Status>>,
{
    match fut().await {
        Ok(resp) => {
            if let Err(err) = tx.send(Ok(resp)).await {
                tracing::error!("{}", err);
            }
        }
        Err(status) => {
            if let Err(err) = tx.send(Err(status)).await {
                tracing::error!("{}", err);
            }
        }
    }
}

async fn create_user(
    request: CreateUserRequest,
    control_service: Arc<ControlService>,
    req_user_uuid: Uuid,
) -> Result<GameUpdate, Status> {
    let user_role = match request.role() {
        ProtoUserRole::RoleUnspecified => {
            return Err(Status::invalid_argument(
                "The user role needs to either be 'User' or 'Admin'",
            ));
        }
        ProtoUserRole::User => UserRole::User,
        ProtoUserRole::Admin => UserRole::Admin,
    };

    match control_service
        .create_user(
            req_user_uuid,
            request.username,
            request.password,
            user_role.clone(),
        )
        .await
    {
        Ok(user) => Ok(GameUpdate {
            response_enum: Some(ResponseEnum::CreateUser(CreateUserResponse {
                uuid: user.uuid.to_string(),
                name: user.name,
                role: user_role.into(),
            })),
        }),
        Err(err) => Err(control_error_into_status(err)),
    }
}

async fn delete_user(
    request: DeleteUserRequest,
    control_service: Arc<ControlService>,
) -> Result<GameUpdate, Status> {
    let Ok(user_uuid) = Uuid::parse_str(&request.uuid) else {
        return Err(Status::invalid_argument("Failed to parse user uuid"));
    };

    match control_service.delete_user(user_uuid).await {
        Ok(_) => Ok(GameUpdate {
            response_enum: Some(ResponseEnum::DeleteUser(DeleteUserResponse {})),
        }),
        Err(err) => Err(control_error_into_status(err)),
    }
}

fn control_error_into_status(err: ServiceError) -> Status {
    match err {
        ServiceError::WrongUserCredentials => Status::unauthenticated(err.to_string()),
        ServiceError::Unauthorized => Status::permission_denied(err.to_string()),
        ServiceError::ControlDatabase(_)
        | ServiceError::EconomyDatabase(_)
        | ServiceError::WarfareDatabase(_)
        | ServiceError::Other(_) => Status::internal(err.to_string()),
    }
}

fn uuid_from_metadata(metadata: &MetadataMap) -> Result<Uuid, Status> {
    let Some(uuid_metadata) = metadata.get(USER_UUID_KEY) else {
        return Err(Status::new(Code::NotFound, "Failed to retrieve user id"));
    };

    let Ok(uuid_str) = uuid_metadata.to_str() else {
        return Err(Status::internal("Failed to parse uuid metadata as string"));
    };

    parse_uuid(uuid_str)
}
