use super::grpc::GrpcHandler;
use crate::domain::{
    auth::{
        AuthenticationRepository, LoginUserReq, RegisterUserReq, ResendVerificationReq,
        VerifyUserReq,
    },
    response::{Response, ResponseType},
};
use syndicode_proto::syndicode_interface_v1::{
    LoginRequest, RegisterRequest, ResendVerificationEmailRequest, VerifyUserRequest,
};
use time::OffsetDateTime;
use tonic::Request;

impl AuthenticationRepository for GrpcHandler {
    async fn register_user(&mut self, req: RegisterUserReq) -> anyhow::Result<Response> {
        let mut request = Request::new(RegisterRequest {
            user_name: req.user_name,
            user_password: req.user_password,
            email: req.email,
            corporation_name: req.corporation_name,
        });

        self.add_ip_metadata(request.metadata_mut())?;

        let result = self.auth_client.register(request).await;

        match result {
            Ok(response) => Ok(Response::builder()
                .response_type(ResponseType::Success)
                .code("OK".to_string())
                .message(format!("{:#?}", response))
                .timestamp(OffsetDateTime::now_utc())
                .build()),
            Err(status) => Ok(Response::builder()
                .response_type(ResponseType::Error)
                .code(status.code().description().to_string())
                .message(format!("{:#?}", status.message()))
                .timestamp(OffsetDateTime::now_utc())
                .build()),
        }
    }

    async fn verifiy_user(&mut self, req: VerifyUserReq) -> anyhow::Result<Response> {
        let mut request = Request::new(VerifyUserRequest {
            user_name: req.user_name,
            code: req.code,
        });

        self.add_ip_metadata(request.metadata_mut())?;

        let result = self.auth_client.verify_user(request).await;

        match result {
            Ok(response) => Ok(Response::builder()
                .response_type(ResponseType::Success)
                .code("OK".to_string())
                .message(format!("{:#?}", response))
                .timestamp(OffsetDateTime::now_utc())
                .build()),
            Err(status) => Ok(Response::builder()
                .response_type(ResponseType::Error)
                .code(status.code().description().to_string())
                .message(format!("{:#?}", status.message()))
                .timestamp(OffsetDateTime::now_utc())
                .build()),
        }
    }

    async fn resend_verification(
        &mut self,
        req: ResendVerificationReq,
    ) -> anyhow::Result<Response> {
        let mut request = Request::new(ResendVerificationEmailRequest {
            user_name: req.user_name,
        });

        self.add_ip_metadata(request.metadata_mut())?;

        let result = self.auth_client.resend_verification_email(request).await;

        match result {
            Ok(response) => Ok(Response::builder()
                .response_type(ResponseType::Success)
                .code("OK".to_string())
                .message(format!("{:#?}", response))
                .timestamp(OffsetDateTime::now_utc())
                .build()),
            Err(status) => Ok(Response::builder()
                .response_type(ResponseType::Error)
                .code(status.code().description().to_string())
                .message(format!("{:#?}", status.message()))
                .timestamp(OffsetDateTime::now_utc())
                .build()),
        }
    }

    async fn login_user(&mut self, req: LoginUserReq) -> anyhow::Result<(String, Response)> {
        let mut request = Request::new(LoginRequest {
            user_name: req.user_name,
            user_password: req.user_password,
        });

        self.add_ip_metadata(request.metadata_mut())?;

        let result = self.auth_client.login(request).await;

        match result {
            Ok(response) => {
                let message = format!("{:#?}", response);
                let jwt = response.into_inner().jwt;

                Ok((
                    jwt,
                    Response::builder()
                        .response_type(ResponseType::Success)
                        .code("OK".to_string())
                        .message(message)
                        .timestamp(OffsetDateTime::now_utc())
                        .build(),
                ))
            }
            Err(status) => Ok((
                "".to_string(),
                Response::builder()
                    .response_type(ResponseType::Error)
                    .code(status.code().description().to_string())
                    .message(format!("{:#?}", status.message()))
                    .timestamp(OffsetDateTime::now_utc())
                    .build(),
            )),
        }
    }
}
