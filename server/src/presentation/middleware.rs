use crate::domain::model::control::Claims;
use http::HeaderValue;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Instant;
use tonic::Status;
use tower::{BoxError, Layer, Service};

pub const USER_UUID_KEY: &str = "user_uuid";
pub const AUTHORIZATION_KEY: &str = "authorization";

const AUTH_EXCEPTED_PATHS: [&str; 2] = [
    "/control.Control/Login",
    "/grpc.reflection.v1.ServerReflection/ServerReflectionInfo",
];

#[derive(Debug, Clone, Default)]
pub struct JwtAuthLayer {
    secret: String,
}

impl JwtAuthLayer {
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),
        }
    }
}

impl<S> Layer<S> for JwtAuthLayer {
    type Service = JwtAuthMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        JwtAuthMiddleware {
            inner: service,
            secret: Arc::new(self.secret.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JwtAuthMiddleware<S> {
    inner: S,
    secret: Arc<String>,
}

type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for JwtAuthMiddleware<S>
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
        let secret = Arc::clone(&self.secret);

        let path = req.uri().path().to_string(); // clone for move
        let start_time = Instant::now();

        Box::pin(async move {
            let skip_auth = AUTH_EXCEPTED_PATHS.contains(&path.as_str());

            tracing::info!(%path, %skip_auth, "Incoming request");

            // Skip auth for exceptional paths
            if skip_auth {
                let response = inner.call(req).await.map_err(Into::into)?;

                tracing::info!(%path, elapsed_ms = start_time.elapsed().as_millis(), "Request completed");

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
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::new(Algorithm::HS256),
            )
            .map_err(|_| Status::unauthenticated("Invalid or expired token"))?;

            // Inject UUID
            let user_uuid = &token_data.claims.sub;
            if let Ok(uuid_header) = HeaderValue::from_str(user_uuid) {
                req.headers_mut().insert(USER_UUID_KEY, uuid_header);
            }

            // Call inner service
            match inner.call(req).await {
                Ok(res) => {
                    tracing::info!(
                        %path,
                        %user_uuid,
                        elapsed_ms = start_time.elapsed().as_millis(),
                        "Request succeeded"
                    );
                    Ok(res)
                }
                Err(err) => {
                    tracing::error!(
                        %path,
                        %user_uuid,
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
