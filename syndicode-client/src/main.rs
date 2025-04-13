use syndicode_proto::{
    syndicode_interface_v1::{
        auth_service_client::AuthServiceClient, game_service_client::GameServiceClient,
        game_update::Update, player_action::Action, LoginRequest, PlayerAction,
    },
    syndicode_warfare_v1::{ListUnitsRequest, SpawnUnitRequest},
};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{metadata::MetadataValue, Request};

pub const SOCKET_ADDR: &str = "[::]:50051";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game_client = GameServiceClient::connect(format!("http://{}", SOCKET_ADDR)).await?;
    let mut auth_client = AuthServiceClient::connect(format!("http://{}", SOCKET_ADDR)).await?;

    // Login
    let login_req = Request::new(LoginRequest {
        user_name: "admin".to_string(),
        user_password: "my-great-password".to_string(),
    });

    let login_resp = auth_client.login(login_req).await?;
    let jwt = &login_resp.get_ref().jwt;
    println!("JWT: {jwt}");

    // Create mpsc channel for sending UserRequests
    let (tx, rx) = mpsc::channel(16);
    let input_stream = ReceiverStream::new(rx);

    let mut stream_request = Request::new(input_stream);
    let bearer = format!("Bearer {jwt}");
    let meta_val = MetadataValue::try_from(bearer.as_str())?;
    stream_request
        .metadata_mut()
        .insert("authorization", meta_val);

    // Start the streaming RPC
    let mut stream = game_client.play_stream(stream_request).await?.into_inner();

    let create_user_request = PlayerAction {
        action: Some(Action::SpawnUnit(SpawnUnitRequest {})),
    };

    // Send initial CreateUser request
    tx.send(create_user_request.clone()).await?;

    // Clone tx for sending DeleteUser later
    let tx_clone = tx.clone();

    // Read from the server
    while let Some(game_update) = stream.next().await {
        match game_update {
            Ok(game_update) => {
                if let Some(update) = game_update.update {
                    match update {
                        Update::ListUnits(list_unit_response) => {
                            println!("List unit reponse: {:?}", list_unit_response);
                        }
                        _ => {
                            println!("Game update: {:?}", update);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Stream error: {e}");
                break;
            }
        }
    }

    Ok(())
}
