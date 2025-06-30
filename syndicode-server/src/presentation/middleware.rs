use crate::application::ports::crypto::JwtHandler;
use crate::application::ports::limiter::{LimiterCategory, RateLimitEnforcer};
use crate::config::ServerConfig;
use crate::presentation::common::limitation_error_into_status;
use http::{HeaderValue, Request, Response};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Instant;
use tonic::Status;
use tower::{BoxError, Layer, Service};

const PROXY_IP_ADDRESS_HEADER: &str = "proxy-ip-address";
const PROXY_API_KEY_HEADER: &str = "proxy-api-key";
pub(super) const USER_IP_ADDRESS_KEY: &str = "user-ip-address";
pub(super) const USER_UUID_KEY: &str = "user-uuid";
pub const AUTHORIZATION_HEADER: &str = "authorization";
const HEALTH_CHECK_PATH: &str = "/grpc.health.v1.Health/Check";

static AUTH_EXCEPTED_PATHS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "/grpc.reflection.v1.ServerReflection/ServerReflectionInfo",
        HEALTH_CHECK_PATH,
        "/syndicode_interface_v1.AuthService/Register",
        "/syndicode_interface_v1.AuthService/VerifyUser",
        "/syndicode_interface_v1.AuthService/ResendVerificationEmail",
        "/syndicode_interface_v1.AuthService/Login",
    ]
    .iter()
    .cloned()
    .collect()
});

struct MiddlewareState<J, R> {
    ip_header_name: String,
    proxy_api_key: String,
    jwt: Arc<J>,
    limit: Arc<R>,
}

#[derive(Clone)]
pub struct MiddlewareLayer<J, R> {
    state: Arc<MiddlewareState<J, R>>,
}

impl<J, R> MiddlewareLayer<J, R>
where
    J: JwtHandler + Clone,
    R: RateLimitEnforcer + Clone,
{
    pub fn new(config: Arc<ServerConfig>, jwt: Arc<J>, limit: Arc<R>) -> Self {
        Self {
            state: Arc::new(MiddlewareState {
                ip_header_name: config.rate_limiter.ip_address_header.clone(),
                proxy_api_key: config.rate_limiter.proxy_api_key.clone(),
                jwt,
                limit,
            }),
        }
    }
}

impl<S, J, R> Layer<S> for MiddlewareLayer<J, R>
where
    J: JwtHandler + Clone,
    R: RateLimitEnforcer + Clone,
{
    type Service = Middleware<S, J, R>;

    fn layer(&self, service: S) -> Self::Service {
        Middleware {
            inner: service,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Middleware<S, J, R> {
    inner: S,
    state: Arc<MiddlewareState<J, R>>,
}

impl<S, J, R, ReqBody, ResBody> Service<Request<ReqBody>> for Middleware<S, J, R>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Error: Into<BoxError> + Send + Sync + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    J: JwtHandler + Clone + Send + Sync + 'static,
    R: RateLimitEnforcer + Clone + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let mut inner = self.inner.clone();
        let state = self.state.clone();

        Box::pin(async move {
            let path = req.uri().path().to_string();

            if path == HEALTH_CHECK_PATH {
                return inner.call(req).await.map_err(Into::into);
            }

            let start_time = Instant::now();

            let ip_header_name: &str = {
                if let Some(key) = req
                    .headers()
                    .get(PROXY_API_KEY_HEADER)
                    .and_then(|h| h.to_str().ok())
                {
                    if key == state.proxy_api_key.as_str() {
                        PROXY_IP_ADDRESS_HEADER
                    } else {
                        &state.ip_header_name
                    }
                } else {
                    &state.ip_header_name
                }
            };

            let ip_address = req
                .headers()
                .get(ip_header_name)
                .and_then(|h| h.to_str().ok())
                .ok_or_else(|| {
                    tracing::warn!("Failed to get IP from header '{}'", ip_header_name);
                    Status::invalid_argument("Missing required client identification")
                })?
                .to_string();

            let ip_address_header_value = HeaderValue::from_str(&ip_address).map_err(|e| {
                tracing::error!(ip_address = %ip_address, error = ?e, "Failed to create HeaderValue");
                Status::internal("Internal server error")
            })?;
            req.headers_mut()
                .insert(USER_IP_ADDRESS_KEY, ip_address_header_value);

            state
                .limit
                .check(LimiterCategory::Middleware, ip_address.as_str())
                .await
                .map_err(limitation_error_into_status)?;

            tracing::info!(method = %req.method(), uri = %req.uri(), ip = %ip_address, action = "request_start");

            let user_uuid_opt: Option<String> = {
                let skip_auth = AUTH_EXCEPTED_PATHS.contains(path.as_str());
                if skip_auth {
                    None
                } else {
                    let token = req
                        .headers()
                        .get(AUTHORIZATION_HEADER)
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.strip_prefix("Bearer "))
                        .ok_or_else(|| {
                            Status::unauthenticated("Missing or malformed Bearer token")
                        })?;

                    let token_data = state.jwt.decode_jwt(token).map_err(|e| {
                        tracing::warn!(error = ?e, "JWT decoding failed");
                        Status::unauthenticated("Invalid token")
                    })?;

                    let user_uuid = token_data.claims.sub;
                    let header_value = HeaderValue::from_str(&user_uuid).map_err(|e| {
                        tracing::error!(user_uuid = %user_uuid, error = ?e, "Failed to create HeaderValue");
                        Status::internal("Internal server error")
                    })?;

                    req.headers_mut().insert(USER_UUID_KEY, header_value);
                    Some(user_uuid)
                }
            };

            // Call the inner service
            let response = inner.call(req).await.map_err(Into::into);

            let elapsed_ms = start_time.elapsed().as_millis();
            match &response {
                Ok(res) => {
                    tracing::info!(
                        status = %res.status(),
                        user_uuid = user_uuid_opt.as_deref().unwrap_or("anonymous"),
                        elapsed_ms,
                        path = %path,
                        action = "request_success",
                    );
                }
                Err(err) => {
                    tracing::error!(
                        user_uuid = user_uuid_opt.as_deref().unwrap_or("anonymous"),
                        elapsed_ms,
                        error = %err,
                        path = %path,
                        action = "request_failure",
                    );
                }
            }

            response
        })
    }
}
