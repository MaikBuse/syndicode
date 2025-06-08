mod common;

use common::setup_test_suite;
use syndicode_client::domain::{
    admin::{AdminRepository, CreateUserDomainRequest, DeleteUserDomainRequest},
    auth::repository::{AuthenticationRepository, LoginUserReq},
    game::GameRepository,
};
use syndicode_proto::{
    syndicode_economy_v1::GetCorporationRequest,
    syndicode_interface_v1::{
        game_update::Update, player_action::Action, GameUpdate, PlayerAction,
    },
};
use tokio::{sync::mpsc::Sender, task::JoinHandle};
use tokio_stream::StreamExt;
use tonic::{Status, Streaming};
use uuid::Uuid;

#[tokio::test]
async fn happy_path_integration_test() {
    tracing_subscriber::fmt::init();

    tracing::info!("Set up test suite");

    let mut test_suite = setup_test_suite().await.unwrap();

    tracing::info!("Login with default admin user");

    let admin_login_response = test_suite
        .grpc_handler
        .login_user(LoginUserReq {
            user_name: test_suite.config.grpc.user_name.clone(),
            user_password: test_suite.config.grpc.user_password,
        })
        .await
        .unwrap();

    tracing::info!("Setup the play stream using the admin token");

    let admin_stream = test_suite
        .grpc_handler
        .play_stream(admin_login_response.jwt.clone())
        .await
        .unwrap();

    // Setup stream
    let (admin_stream_result_tx, mut admin_stream_result_rx) =
        tokio::sync::mpsc::channel::<Result<GameUpdate, Status>>(10);

    let admin_stream_handle = spawn_stream_thread(admin_stream, admin_stream_result_tx);

    tracing::info!("Create a new player using the admin token");

    let player_name = "Player_Name".to_string();
    let player_password = "Player_Password".to_string();
    let player_email = "player@email.com".to_string();
    let player_role = 2_i32;
    let player_corporation = "Player-Corp".to_string();

    let player_create_request_uuid = Uuid::now_v7();

    let player_create_response = test_suite
        .grpc_handler
        .create_user(
            admin_login_response.jwt.clone(),
            CreateUserDomainRequest {
                request_uuid: player_create_request_uuid.to_string(),
                user_name: player_name.clone(),
                user_password: player_password.clone(),
                user_email: player_email,
                user_role: player_role,
                corporation_name: player_corporation.clone(),
            },
        )
        .await
        .unwrap();

    tracing::info!("Wait until the corporation has been created and the notification is received from the stream");

    'while_player_corp_create_stream: while let Some(result) = admin_stream_result_rx.recv().await {
        match result {
            Ok(game_update) => {
                match game_update.clone().update.unwrap() {
                    Update::TickNotification(_) => continue 'while_player_corp_create_stream,
                    Update::CreateCorporation(resp) => {
                        assert_eq!(player_create_request_uuid.to_string(), resp.request_uuid);

                        let Some(corporation) = resp.corporation else {
                            panic!("Failed to retrieve corporation from CreateCorporation stream update");
                        };

                        assert_eq!(player_corporation, corporation.name);
                        assert_eq!(player_create_response.user_uuid, corporation.user_uuid);

                        tracing::info!("Player corporation created and verified - breaking loop");

                        break 'while_player_corp_create_stream;
                    }
                    _ => panic!("Received unexpected game update: {:?}", game_update),
                }
            }
            Err(status) => panic!("Received unexpected status: {:?}", status),
        };
    }

    tracing::info!("Close the admin's stream");

    drop(admin_stream_result_rx);
    drop(admin_stream_handle);

    tracing::info!("Login with the player user");

    let player_login_response = test_suite
        .grpc_handler
        .login_user(LoginUserReq {
            user_name: player_name,
            user_password: player_password,
        })
        .await
        .unwrap();

    tracing::info!("Setup the play stream with the player's token");

    let player_stream = test_suite
        .grpc_handler
        .play_stream(player_login_response.jwt.clone())
        .await
        .unwrap();

    // Setup stream
    let (player_stream_result_tx, mut player_stream_result_rx) =
        tokio::sync::mpsc::channel::<Result<GameUpdate, Status>>(10);

    spawn_stream_thread(player_stream, player_stream_result_tx);

    tracing::info!("Get the corporation of the created player");

    // Send request through stream
    let request_uuid = Uuid::now_v7().to_string();
    let client_action_tx = test_suite
        .grpc_handler
        .maybe_client_action_tx
        .clone()
        .unwrap();
    client_action_tx
        .send(PlayerAction {
            request_uuid: request_uuid.clone(),
            action: Some(Action::GetCorporation(GetCorporationRequest {})),
        })
        .await
        .unwrap();

    tracing::info!("Wait until the corporation is returned from the stream");

    // Handle stream responses
    let mut received_action_init = false;

    'while_player_corp_get_stream: while let Some(result) = player_stream_result_rx.recv().await {
        match result {
            Ok(game_update) => match game_update.clone().update.unwrap() {
                Update::ActionInitResponse(action_init_response) => {
                    if received_action_init {
                        panic!("Received a second action init response which was not expected");
                    }

                    match action_init_response.request_uuid == request_uuid {
                        true => received_action_init = true,
                        false => panic!("Request uuid is not as expected"),
                    }
                }
                Update::TickNotification(_) => continue 'while_player_corp_get_stream,
                Update::GetCorporation(get_corporation_response) => {
                    assert_eq!(get_corporation_response.request_uuid, request_uuid);

                    let corporation = get_corporation_response.corporation.unwrap();

                    assert_eq!(player_corporation, corporation.name);
                    assert_eq!(player_create_response.user_uuid, corporation.user_uuid);

                    tracing::info!("Player corporation has been read and verified - breaking loop");

                    break 'while_player_corp_get_stream;
                }
                _ => panic!("Received unexpected game update: {:?}", game_update),
            },
            Err(status) => panic!("Received unexpected status: {:?}", status),
        };
    }

    tracing::info!("Delete the player user by the player itself");

    let player_delete_request_uuid = Uuid::now_v7();

    let delete_user_req = DeleteUserDomainRequest {
        request_uuid: player_delete_request_uuid.to_string(),
        user_uuid: player_create_response.user_uuid.clone(),
    };

    let player_delete_result = test_suite
        .grpc_handler
        .delete_user(player_login_response.jwt, delete_user_req)
        .await;

    let Ok(player_delete_response) = player_delete_result else {
        panic!("Failed to delete player user: {:?}", player_delete_result);
    };

    assert_eq!(
        player_create_response.user_uuid,
        player_delete_response.user_uuid
    );

    tracing::info!(
        "Wait until the player's corporation has been deleted and the stream returns the update"
    );

    'while_player_corp_delete_stream: while let Some(result) = player_stream_result_rx.recv().await
    {
        match result {
            Ok(game_update) => match game_update.clone().update.unwrap() {
                Update::TickNotification(_) => continue 'while_player_corp_delete_stream,
                Update::DeleteCorporation(resp) => {
                    assert_eq!(player_delete_request_uuid.to_string(), resp.request_uuid);

                    assert_eq!(player_delete_response.user_uuid, resp.user_uuid);

                    tracing::info!(
                        "Player corporation deleted and response verified - breaking loop"
                    );

                    break 'while_player_corp_delete_stream;
                }
                _ => panic!("Received unexpected game update: {:?}", game_update),
            },
            Err(status) => panic!("Received unexpected status: {:?}", status),
        };
    }
}

fn spawn_stream_thread(
    mut stream: Streaming<GameUpdate>,
    tx: Sender<Result<GameUpdate, Status>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(result) = stream.next().await {
            let _ = tx.send(result).await;
        }
    })
}
