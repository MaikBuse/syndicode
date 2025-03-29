use server::{
    infrastructure::postgres::ADMIN_USERNAME,
    presentation::proto::control::{control_client::ControlClient, LoginRequest},
    ADMIN_PASSWORD_ENV, SOCKET_ADDR,
};

#[tokio::test]
async fn login_with_default_admin() {
    let mut control_client = ControlClient::connect(format!("http://{}", SOCKET_ADDR))
        .await
        .unwrap();

    let password = std::env::var(ADMIN_PASSWORD_ENV).unwrap();

    let request = tonic::Request::new(LoginRequest {
        username: ADMIN_USERNAME.to_string(),
        password,
    });

    control_client.login(request).await.unwrap();
}
