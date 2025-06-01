mod common;

use common::setup_test_suite;
use syndicode_client::domain::auth::repository::{AuthenticationRepository, LoginUserReq};

#[tokio::test]
async fn should_get_current_user() {
    let mut test_suite = setup_test_suite().await.unwrap();

    let login_response = test_suite
        .grpc_handler
        .login_user(LoginUserReq {
            user_name: test_suite.config.grpc.user_name.clone(),
            user_password: test_suite.config.grpc.user_password,
        })
        .await
        .unwrap();

    let response = test_suite
        .grpc_handler
        .get_current_user(login_response.jwt)
        .await
        .unwrap();

    assert_eq!(test_suite.config.grpc.user_name, response.user_name);
    assert_eq!("Active".to_string(), response.status);
}
