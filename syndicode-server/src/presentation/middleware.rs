use crate::domain::model::control::Claims;
use governor::{clock::DefaultClock, state::keyed::DefaultKeyedStateStore, RateLimiter};
use http::HeaderValue;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Instant;
use tonic::Status;
use tower::{BoxError, Layer, Service};
use uuid::Uuid;

pub type UserLimiter = RateLimiter<Uuid, DefaultKeyedStateStore<Uuid>, DefaultClock>;

pub const USER_UUID_KEY: &str = "user_uuid";
pub const AUTHORIZATION_KEY: &str = "authorization";

const HEALTH_CHECK_PATH: &str = "/grpc.health.v1.Health/Check";

const AUTH_EXCEPTED_PATHS: [&str; 4] = [
    "/grpc.reflection.v1.ServerReflection/ServerReflectionInfo",
    HEALTH_CHECK_PATH,
    "/control.Control/Register",
    "/control.Control/Login",
];

#[derive(Debug, Clone)]
pub struct MiddlewareLayer {
    jwt_secret: String,
    user_limiter: Arc<UserLimiter>,
}

impl MiddlewareLayer {
    pub fn new(jwt_secret: String, user_limiter: Arc<UserLimiter>) -> Self {
        Self {
            jwt_secret,
            user_limiter,
        }
    }
}

impl<S> Layer<S> for MiddlewareLayer {
    type Service = Middleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        Middleware {
            inner: service,
            jwt_secret: Arc::new(self.jwt_secret.clone()),
            user_limiter: Arc::clone(&self.user_limiter),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Middleware<S> {
    inner: S,
    jwt_secret: Arc<String>,
    user_limiter: Arc<UserLimiter>,
}

type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for Middleware<S>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<BoxError> + Send + std::fmt::Debug + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut req: http::Request<ReqBody>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let jwt_secret = Arc::clone(&self.jwt_secret);
        let user_limiter = Arc::clone(&self.user_limiter);

        let path = req.uri().path().to_string(); // clone for move
        let start_time = Instant::now();

        Box::pin(async move {
            let skip_auth = AUTH_EXCEPTED_PATHS.contains(&path.as_str());
            let skip_logging = path.as_str() == HEALTH_CHECK_PATH;

            if !skip_logging {
                tracing::info!(%path, %skip_auth, "Incoming request");
            }

            // Skip auth for exceptional paths
            if skip_auth {
                let response = inner.call(req).await.map_err(Into::into)?;

                if !skip_logging {
                    tracing::info!(%path, elapsed_ms = start_time.elapsed().as_millis(), "Request completed");
                }

                return Ok(response);
            }

            // Extract Bearer token
            let token = req
                .headers()
                .get(AUTHORIZATION_KEY)
                .and_then(|val| val.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "))
                .ok_or_else(|| Status::unauthenticated("Missing or malformed Bearer token"))?;

            // Decode JWT
            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::new(Algorithm::HS256),
            )
            .map_err(|_| Status::unauthenticated("Invalid or expired token"))?;

            // Inject UUID
            let user_uuid_str = &token_data.claims.sub;

            // Check rate limit
            match Uuid::parse_str(user_uuid_str) {
                Ok(user_uuid) => {
                    if user_limiter.check_key(&user_uuid).is_err() {
                        tracing::warn!(%user_uuid, "Rate limit exceeded");

                        return Err(Status::resource_exhausted("Rate limit exceeded").into());
                    }
                }
                Err(_) => {
                    return Err(Status::unauthenticated("Invalid sub in token claims").into());
                }
            }

            if let Ok(uuid_header) = HeaderValue::from_str(user_uuid_str) {
                req.headers_mut().insert(USER_UUID_KEY, uuid_header);
            }

            // Call inner service
            match inner.call(req).await {
                Ok(res) => {
                    if !skip_logging {
                        tracing::info!(
                            %path,
                            %user_uuid_str,
                            elapsed_ms = start_time.elapsed().as_millis(),
                            "Request succeeded"
                        );
                    }
                    Ok(res)
                }
                Err(err) => {
                    tracing::error!(
                        %path,
                        %user_uuid_str,
                        elapsed_ms = start_time.elapsed().as_millis(),
                        error = ?err,
                        "Request failed"
                    );
                    Err(err.into())
                }
            }
        })
    }
}
