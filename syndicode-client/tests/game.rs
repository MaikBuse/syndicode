mod common;

use common::{login_and_setup_stream, setup_test_suite, wait_for_stream_update};
use syndicode_client::domain::admin::{
    AdminRepository, CreateUserDomainRequest, DeleteUserDomainRequest,
};
use syndicode_proto::{
    syndicode_economy_v1::GetCorporationRequest,
    syndicode_interface_v1::{game_update::Update, player_action::Action, PlayerAction},
};
use uuid::Uuid;

#[tokio::test]
async fn happy_path_integration_test() {
    tracing_subscriber::fmt::init();

    tracing::info!("Set up test suite");
    let mut test_suite = setup_test_suite().await.unwrap();

    tracing::info!("Admin logs in and sets up stream");
    let mut admin = login_and_setup_stream(
        &mut test_suite.grpc_handler,
        &test_suite.config.grpc.user_name,
        &test_suite.config.grpc.user_password,
    )
    .await
    .expect("Failed to log in as admin");

    tracing::info!("Create a new player using the admin token");
    let player_name = "Player_Name".to_string();
    let player_password = "Player_Password".to_string();
    let player_corporation = "Player-Corp".to_string();
    let player_create_request_uuid = Uuid::now_v7();

    let player_create_response = test_suite
        .grpc_handler
        .create_user(
            admin.jwt.clone(),
            CreateUserDomainRequest {
                request_uuid: player_create_request_uuid.to_string(),
                user_name: player_name.clone(),
                user_password: player_password.clone(),
                user_email: "player@email.com".to_string(),
                user_role: 2,
                corporation_name: player_corporation.clone(),
            },
        )
        .await
        .expect("Admin failed to create player");

    tracing::info!("Wait for corporation creation notification on admin stream");
    let create_corp_update = wait_for_stream_update(
        &mut admin.stream_rx,
        "CreateCorporation notification",
        |update| match update.update {
            Some(Update::CreateCorporation(resp)) => Some(resp),
            _ => None,
        },
    )
    .await;

    assert_eq!(
        player_create_request_uuid.to_string(),
        create_corp_update.request_uuid
    );
    let created_corp = create_corp_update
        .corporation
        .expect("Corporation data missing from stream update");
    assert_eq!(player_corporation, created_corp.name);
    assert_eq!(player_create_response.user_uuid, created_corp.user_uuid);
    tracing::info!("Admin received correct corporation creation notification");

    // Admin stream is closed automatically when `admin` goes out of scope.
    drop(admin);

    // --- 2. Player logs in and interacts with the game ---
    tracing::info!("Player logs in and sets up stream");
    let mut player =
        login_and_setup_stream(&mut test_suite.grpc_handler, &player_name, &player_password)
            .await
            .expect("Failed to log in as player");

    // The created player's UUID should match the one from the creation response
    assert_eq!(player.user_uuid, player_create_response.user_uuid);

    tracing::info!("Player requests their corporation data");
    let get_corp_request_uuid = Uuid::now_v7().to_string();
    let client_action_tx = test_suite
        .grpc_handler
        .maybe_client_action_tx
        .as_ref()
        .unwrap();
    client_action_tx
        .send(PlayerAction {
            request_uuid: get_corp_request_uuid.clone(),
            action: Some(Action::GetCorporation(GetCorporationRequest {})),
        })
        .await
        .unwrap();

    let get_corp_update =
        wait_for_stream_update(&mut player.stream_rx, "GetCorporation response", |update| {
            match update.update {
                Some(Update::GetCorporation(resp)) => Some(resp),
                // We can also ignore ActionInitResponse here if we don't need to assert on it
                Some(Update::ActionInitResponse(_)) => None,
                _ => None,
            }
        })
        .await;

    assert_eq!(get_corp_update.request_uuid, get_corp_request_uuid);
    let received_corp = get_corp_update.corporation.unwrap();
    assert_eq!(player_corporation, received_corp.name);
    assert_eq!(player.user_uuid, received_corp.user_uuid);
    tracing::info!("Player received correct corporation data");

    // --- 3. Player deletes their own account ---
    tracing::info!("Player deletes their own user");
    let player_delete_request_uuid = Uuid::now_v7();
    let player_delete_response = test_suite
        .grpc_handler
        .delete_user(
            player.jwt.clone(),
            DeleteUserDomainRequest {
                request_uuid: player_delete_request_uuid.to_string(),
                user_uuid: player.user_uuid.clone(),
            },
        )
        .await
        .expect("Player failed to delete their own user");

    assert_eq!(player.user_uuid, player_delete_response.user_uuid);

    let delete_corp_update = wait_for_stream_update(
        &mut player.stream_rx,
        "DeleteCorporation notification",
        |update| match update.update {
            Some(Update::DeleteCorporation(resp)) => Some(resp),
            _ => None,
        },
    )
    .await;

    assert_eq!(
        player_delete_request_uuid.to_string(),
        delete_corp_update.request_uuid
    );
    assert_eq!(player.user_uuid, delete_corp_update.user_uuid);
    tracing::info!("Player received correct corporation deletion notification");
}
