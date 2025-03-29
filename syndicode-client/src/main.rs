use syndicode_proto::control::{
    control_client::ControlClient, game_update::ResponseEnum, user_request::RequestEnum,
    CreateUserRequest, DeleteUserRequest, LoginRequest, UserRequest,
};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{metadata::MetadataValue, Request};

pub const SOCKET_ADDR: &str = "[::]:50051";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut control_client = ControlClient::connect(format!("http://{}", SOCKET_ADDR)).await?;

    // Login
    let login_req = Request::new(LoginRequest {
        username: "admin".to_string(),
        password: "my-great-password".to_string(),
    });

    let login_resp = control_client.login(login_req).await?;
    let jwt = &login_resp.get_ref().jwt;
    println!("JWT: {jwt}");

    // Create mpsc channel for sending UserRequests
    let (tx, rx) = mpsc::channel(32);
    let input_stream = ReceiverStream::new(rx);

    let mut stream_request = Request::new(input_stream);
    let bearer = format!("Bearer {jwt}");
    let meta_val = MetadataValue::try_from(bearer.as_str())?;
    stream_request
        .metadata_mut()
        .insert("authorization", meta_val);

    // Start the streaming RPC
    let mut stream = control_client
        .game_stream_rpc(stream_request)
        .await?
        .into_inner();

    // Username we want to create & delete
    let target_username = "user";

    let create_user_request = UserRequest {
        request_enum: Some(RequestEnum::CreateUser(CreateUserRequest {
            username: target_username.to_string(),
            password: "new-user-pw".to_string(),
            role: 2,
        })),
    };

    // Send initial CreateUser request
    tx.send(create_user_request.clone()).await?;

    // Clone tx for sending DeleteUser later
    let tx_clone = tx.clone();

    // Read from the server
    while let Some(update) = stream.next().await {
        match update {
            Ok(response) => {
                if let Some(response_enum) = response.response_enum {
                    match response_enum {
                        ResponseEnum::CreateUser(create_user_response) => {
                            println!("User created: {:?}", create_user_response);
                            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

                            // Now delete the user
                            let _ = tx_clone
                                .send(UserRequest {
                                    request_enum: Some(RequestEnum::DeleteUser(
                                        DeleteUserRequest {
                                            uuid: create_user_response.uuid,
                                        },
                                    )),
                                })
                                .await;
                        }
                        ResponseEnum::DeleteUser(delete_user_response) => {
                            println!("User deleted: {:?}", delete_user_response);
                            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

                            let _ = tx_clone.send(create_user_request.clone()).await;
                        }
                        _ => {
                            println!("Game update: {:?}", response_enum);
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
