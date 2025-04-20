use crate::application::ports::crypto::JwtHandler;
use crate::application::ports::limiter::{LimiterCategory, RateLimitEnforcer};
use crate::config::Config;
use crate::presentation::common::limitation_error_into_status;
use http::HeaderValue;
use std::collections::HashSet;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Instant;
use tonic::Status;
use tower::{BoxError, Layer, Service};

pub const USER_UUID_KEY: &str = "user-uuid";
pub const AUTHORIZATION_HEADER: &str = "authorization";

const HEALTH_CHECK_PATH: &str = "/grpc.health.v1.Health/Check";

lazy_static::lazy_static! {
    static ref AUTH_EXCEPTED_PATHS: HashSet<&'static str> = [
        "/grpc.reflection.v1.ServerReflection/ServerReflectionInfo",
        HEALTH_CHECK_PATH,
        "/syndicode_interface_v1.AuthService/Register",
        "/syndicode_interface_v1.AuthService/VerifyUser",
        "/syndicode_interface_v1.AuthService/ResendVerificationEmail",
        "/syndicode_interface_v1.AuthService/Login",
    ]
    .iter()
    .cloned()
    .collect();
}

#[derive(Clone)]
pub struct MiddlewareLayer<J, R>
where
    J: JwtHandler + Clone,
    R: RateLimitEnforcer + Clone,
{
    ip_header_name: Arc<String>,
    jwt: Arc<J>,
    limit: Arc<R>,
    auth_excepted_paths: Arc<HashSet<&'static str>>,
}

impl<J, R> MiddlewareLayer<J, R>
where
    J: JwtHandler + Clone,
    R: RateLimitEnforcer + Clone,
{
    pub fn new(config: Arc<Config>, jwt: Arc<J>, limit: Arc<R>) -> Self {
        let ip_header_name = Arc::new(config.ip_address_header.clone());

        let auth_excepted_paths = Arc::new(AUTH_EXCEPTED_PATHS.clone());
        Self {
            ip_header_name,
            jwt,
            limit,
            auth_excepted_paths,
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
            ip_header_name: Arc::clone(&self.ip_header_name),
            jwt: Arc::clone(&self.jwt),
            limit: Arc::clone(&self.limit),
            auth_excepted_paths: Arc::clone(&self.auth_excepted_paths),
        }
    }
}

#[derive(Clone)]
pub struct Middleware<S, J, R>
where
    J: JwtHandler + Clone,
    R: RateLimitEnforcer + Clone,
{
    inner: S,
    ip_header_name: Arc<String>,
    jwt: Arc<J>,
    limit: Arc<R>,
    auth_excepted_paths: Arc<HashSet<&'static str>>,
}

type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S, J, R, ReqBody, ResBody> Service<http::Request<ReqBody>> for Middleware<S, J, R>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<BoxError> + Send + Sync + std::fmt::Debug + std::fmt::Display + 'static,
    ReqBody: Send + 'static,
    J: JwtHandler + Clone + 'static,        // Add necessary bounds
    R: RateLimitEnforcer + Clone + 'static, // Add necessary bounds
{
    type Response = S::Response;
    type Error = BoxError; // BoxError is often convenient for middleware
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut req: http::Request<ReqBody>) -> Self::Future {
        // Use tower::ServiceExt::oneshot or clone manually like before.
        // Cloning is often simpler to reason about.
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        // Clone Arcs needed for the future
        let jwt = Arc::clone(&self.jwt);
        let limit = Arc::clone(&self.limit);
        let ip_header_name = Arc::clone(&self.ip_header_name);
        let auth_excepted_paths = Arc::clone(&self.auth_excepted_paths);

        Box::pin(async move {
            let start_time = Instant::now();
            let path = req.uri().path().to_string(); // Clone path for logging etc.

            let skip_auth = auth_excepted_paths.contains(path.as_str());
            let is_health_check = path == HEALTH_CHECK_PATH;

            if !is_health_check {
                // IP Address Extraction
                let ip_address = req
                    .headers()
                    .get(ip_header_name.as_str())
                    .and_then(|h| h.to_str().ok())
                    .ok_or_else(|| {
                        tracing::warn!(
                            "Failed to retrieve client IP address from header '{}'",
                            *ip_header_name
                        );

                        Status::invalid_argument("Missing required client identification")
                    })?;

                // Rate Limiting
                if let Err(err) = limit.check(LimiterCategory::Middleware, ip_address).await {
                    return Err(limitation_error_into_status(err).into());
                }

                // Use structured logging
                tracing::info!(
                    method = %req.method(),
                    uri = %req.uri(),
                    version = ?req.version(),
                    ip = %ip_address, // Log IP earlier
                    action = "request_start",
                    auth_skipped = skip_auth,
                );
            }

            // Authorization
            let user_uuid_str_opt: Option<String> = if skip_auth {
                None
            } else {
                // Extract Bearer token
                let token = req
                    .headers()
                    .get(AUTHORIZATION_HEADER)
                    .and_then(|val| val.to_str().ok())
                    .and_then(|s| s.strip_prefix("Bearer "))
                    .ok_or_else(|| Status::unauthenticated("Missing or malformed Bearer token"))?;

                // Decode JWT
                let token_data = jwt.decode_jwt(token).map_err(|e| {
                    tracing::warn!(error = ?e, "JWT decoding failed");

                    Status::unauthenticated("Invalid token")
                })?;

                // Inject UUID
                let user_uuid = token_data.claims.sub; // Assuming this is already a String or easily convertible
                if let Ok(uuid_header) = HeaderValue::from_str(&user_uuid) {
                    req.headers_mut().insert(USER_UUID_KEY, uuid_header);
                } else {
                    // This case should ideally not happen if UUIDs are valid strings
                    tracing::error!(user_uuid = %user_uuid, "Failed to create HeaderValue from user UUID");

                    return Err(Status::internal("Internal processing error").into());
                }
                Some(user_uuid)
            };

            // Call Inner Service
            let response_result = inner.call(req).await;
            let elapsed_ms = start_time.elapsed().as_millis();

            // Response Logging
            match &response_result {
                Ok(res) => {
                    if !is_health_check {
                        tracing::info!(
                            status = %res.status(), // Log HTTP status if available/relevant
                            user_uuid = user_uuid_str_opt.as_deref().unwrap_or("anonymous"),
                            elapsed_ms,
                            action = "request_success",
                            path = %path, // Log path again for context
                        );
                    }
                }
                Err(err) => {
                    tracing::error!(
                        user_uuid = user_uuid_str_opt.as_deref().unwrap_or("anonymous"),
                        elapsed_ms,
                        error = %err,
                        action = "request_failure",
                        path = %path,
                    );
                }
            }

            response_result.map_err(Into::into)
        })
    }
}
