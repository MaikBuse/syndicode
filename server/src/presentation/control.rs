use super::economy::get_corporation;
use super::middleware::USER_UUID_KEY;
use super::proto::control::control_server::Control;
use super::proto::control::user_request::RequestEnum;
use super::proto::control::{CreateUserRequest, GameUpdate, UserRequest};
use super::proto::control::{CreateUserResponse, UserRole as ProtoUserRole};
use super::proto::control::{LoginRequest, LoginResponse};
use super::proto::{
    control::{
        EndGameRequest, EndGameResponse, InitGameRequest, InitGameResponse, JoinGameRequest,
        JoinGameResponse, SessionInfo, StartGameRequest, StartGameResponse,
    },
    economy::CorporationInfo,
};
use super::warfare::{list_units, spawn_unit};
use crate::domain::model::control::UserRole;
use crate::engine::Job;
use crate::service::control::ControlService;
use crate::service::economy::EconomyService;
use crate::service::warfare::WarfareService;
use crate::{
    domain::model::control::SessionState, presentation::proto::control::game_update::ResponseEnum,
    service::control::ControlServiceError,
};
use dashmap::DashMap;
use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::async_trait;
use tonic::metadata::MetadataMap;
use tonic::Streaming;
use tonic::{Code, Request, Response, Status};

type PlayerTx = mpsc::Sender<Result<GameUpdate, Status>>;

pub struct ControlPresenter {
    pub jobs: Arc<DashMap<Vec<u8>, VecDeque<Job>>>,
    pub user_channels: Arc<DashMap<Vec<u8>, PlayerTx>>,
    pub control_service: Arc<ControlService>,
    pub economy_service: Arc<EconomyService>,
    pub warfare_service: Arc<WarfareService>,
}

#[async_trait]
impl Control for ControlPresenter {
    type GameStreamRpcStream = Pin<Box<dyn Stream<Item = Result<GameUpdate, Status>> + Send>>;

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
        let user_uuid = match uuid_from_metadata(request.metadata()) {
            Ok(uuid) => uuid,
            Err(status) => return Err(status),
        };

        let mut stream = request.into_inner();

        // Channel to send game updates to this player
        let (tx, rx) = mpsc::channel(16);
        let player_rx = ReceiverStream::new(rx);

        self.user_channels.insert(user_uuid.clone(), tx.clone());

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
                        RequestEnum::CreateUser(create_user_request) => {
                            match create_user(create_user_request, Arc::clone(&control_service))
                                .await
                            {
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
                        RequestEnum::InitGame(init_game_request) => {
                            match init_game(init_game_request, Arc::clone(&control_service)).await {
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
                        RequestEnum::StartGame(start_game_request) => {
                            match start_game(start_game_request, Arc::clone(&control_service)).await
                            {
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
                        RequestEnum::EndGame(end_game_request) => {
                            match end_game(end_game_request, Arc::clone(&control_service)).await {
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
                        RequestEnum::JoinGame(join_game_request) => {
                            match join_game(
                                join_game_request,
                                Arc::clone(&control_service),
                                user_uuid.clone(),
                            )
                            .await
                            {
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
                        RequestEnum::GetCorporation(get_corporation_request) => {
                            match get_corporation(
                                get_corporation_request,
                                Arc::clone(&economy_service),
                                user_uuid.clone(),
                            )
                            .await
                            {
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
                        RequestEnum::SpawnUnit(spawn_unit_request) => {
                            match spawn_unit(spawn_unit_request, Arc::clone(&jobs)).await {
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
                        RequestEnum::ListUnit(list_units_request) => {
                            match list_units(
                                list_units_request,
                                Arc::clone(&warfare_service),
                                user_uuid.clone(),
                            )
                            .await
                            {
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
                    }
                }
            }
        });

        Ok(Response::new(
            Box::pin(player_rx) as Self::GameStreamRpcStream
        ))
    }
}

async fn create_user(
    request: CreateUserRequest,
    control_service: Arc<ControlService>,
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
        .create_user(request.username, request.password, user_role)
        .await
    {
        Ok(user) => {
            let user_role = match UserRole::try_from(user.role) {
                Ok(user_role) => user_role,
                Err(err) => {
                    tracing::error!("{}", err);

                    return Err(Status::invalid_argument(
                        "The user role needs to either be 'User' or 'Admin'",
                    ));
                }
            };

            Ok(GameUpdate {
                response_enum: Some(ResponseEnum::CreateUser(CreateUserResponse {
                    uuid: user.uuid,
                    name: user.name,
                    role: user_role.into(),
                })),
            })
        }
        Err(err) => Err(control_error_into_status(err)),
    }
}

async fn init_game(
    _request: InitGameRequest,
    control_service: Arc<ControlService>,
) -> Result<GameUpdate, Status> {
    match control_service.create_session().await {
        Ok(session) => {
            let state = match SessionState::try_from(session.state) {
                Ok(state) => state.into(),
                Err(err) => return Err(Status::new(Code::Internal, err.to_string())),
            };

            Ok(GameUpdate {
                response_enum: Some(ResponseEnum::InitGame(InitGameResponse {
                    session: Some(SessionInfo {
                        uuid: session.uuid,
                        interval: session.interval,
                        state,
                    }),
                })),
            })
        }
        Err(err) => Err(control_error_into_status(err)),
    }
}

async fn start_game(
    request: StartGameRequest,
    control_service: Arc<ControlService>,
) -> Result<GameUpdate, Status> {
    if let Err(err) = control_service
        .update_session_state(request.session_uuid, SessionState::Running)
        .await
    {
        return Err(control_error_into_status(err));
    }

    Ok(GameUpdate {
        response_enum: Some(ResponseEnum::StartGame(StartGameResponse {})),
    })
}

async fn end_game(
    request: EndGameRequest,
    control_service: Arc<ControlService>,
) -> Result<GameUpdate, Status> {
    if let Err(err) = control_service.delete_session(request.session_uuid).await {
        return Err(control_error_into_status(err));
    }

    Ok(GameUpdate {
        response_enum: Some(ResponseEnum::EndGame(EndGameResponse {})),
    })
}

async fn join_game(
    request: JoinGameRequest,
    control_service: Arc<ControlService>,
    user_uuid: Vec<u8>,
) -> Result<GameUpdate, Status> {
    let corporation = match control_service
        .join_game(request.session_uuid, user_uuid, request.corporation_name)
        .await
    {
        Ok(corporation) => corporation,
        Err(err) => return Err(control_error_into_status(err)),
    };

    Ok(GameUpdate {
        response_enum: Some(ResponseEnum::JoinGame(JoinGameResponse {
            corporation: Some(CorporationInfo {
                uuid: corporation.uuid,
                session_uuid: corporation.session_uuid,
                user_uuid: corporation.user_uuid,
                name: corporation.name,
                balance: corporation.balance,
            }),
        })),
    })
}

fn control_error_into_status(err: ControlServiceError) -> Status {
    match err {
        ControlServiceError::WrongUserCredentials => Status::unauthenticated(err.to_string()),
        ControlServiceError::SessionAlreadyRunning
        | ControlServiceError::SessionNotRunning
        | ControlServiceError::SessionAlreadyInitialized => {
            Status::invalid_argument(err.to_string())
        }
        ControlServiceError::UuidFromSlice => Status::internal(err.to_string()),
        ControlServiceError::ControlDatabase(_)
        | ControlServiceError::EconomyDatabase(_)
        | ControlServiceError::Other(_) => Status::internal(err.to_string()),
    }
}

fn uuid_from_metadata(metadata: &MetadataMap) -> Result<Vec<u8>, Status> {
    let Some(uuid_metadata) = metadata.get(USER_UUID_KEY) else {
        return Err(Status::new(Code::NotFound, "Failed to retrieve player id"));
    };

    let uuid: Vec<u8> = match uuid_metadata.to_bytes() {
        Ok(uuid) => uuid.into(),
        Err(err) => {
            return Err(Status::new(
                Code::InvalidArgument,
                format!("Failed to retrieve user uuid: {}", err),
            ));
        }
    };

    Ok(uuid)
}
