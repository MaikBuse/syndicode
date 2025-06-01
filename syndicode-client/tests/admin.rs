mod common;

use common::setup_test_suite;
use syndicode_client::domain::{
    admin::{AdminRepository, CreateUserDomainRequest},
    auth::repository::{AuthenticationRepository, LoginUserReq},
};

#[tokio::test]
async fn should_create_and_delete_user() {
    let mut test_suite = setup_test_suite().await.unwrap();

    let login_response = test_suite
        .grpc_handler
        .login_user(LoginUserReq {
            user_name: test_suite.config.grpc.user_name.clone(),
            user_password: test_suite.config.grpc.user_password,
        })
        .await
        .unwrap();

    let user_name = "Test_User".to_string();
    let user_role = 2_i32;
    let email = "test@mail.com".to_string();

    let create_response = test_suite
        .grpc_handler
        .create_user(
            login_response.jwt.clone(),
            CreateUserDomainRequest {
                user_name: user_name.clone(),
                user_password: "Test_Password".to_string(),
                user_email: email.clone(),
                user_role,
                corporation_name: "Test-Corp".to_string(),
            },
        )
        .await
        .unwrap();

    assert_eq!(user_name, create_response.user_name);
    assert_eq!(user_role, create_response.user_role);

    let get_response = test_suite
        .grpc_handler
        .get_user(
            login_response.jwt.clone(),
            create_response.user_uuid.clone(),
        )
        .await
        .unwrap();

    assert_eq!(user_name, get_response.user_name);
    assert_eq!(user_role, get_response.user_role);
    assert_eq!(email, get_response.email);
    assert_eq!(create_response.user_uuid, get_response.user_uuid);
    assert_eq!("Active".to_string(), get_response.status);

    let delete_result = test_suite
        .grpc_handler
        .delete_user(login_response.jwt, create_response.user_uuid)
        .await;

    assert!(delete_result.is_ok());
}
