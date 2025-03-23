use http::{HeaderValue, Request};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tonic::Status;
use tower::{BoxError, Layer, Service};

use crate::domain::model::control::Claims;

pub const USER_UUID_KEY: &str = "user_uuid";
pub const AUTHORIZATION_KEY: &str = "authorization";
const LOGIN_PATH: &str = "/control.Control/Login";

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

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for JwtAuthMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = http::Response<ResBody>, Error = BoxError>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let secret = Arc::clone(&self.secret);

        Box::pin(async move {
            // Skip auth for Login RPC
            let path = req.uri().path();
            if path == LOGIN_PATH {
                return inner.call(req).await.map_err(Into::into);
            }

            // 1. Extract Bearer token
            let token = req
                .headers()
                .get(AUTHORIZATION_KEY)
                .and_then(|val| val.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "))
                .ok_or_else(|| Status::unauthenticated("Missing or malformed Bearer token"))?;

            // 2. Decode JWT
            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::new(Algorithm::HS256),
            )
            .map_err(|_| Status::unauthenticated("Invalid or expired token"))?;

            // 3. Inject UUID into headers (as metadata)
            if let Ok(uuid_header) = HeaderValue::from_str(&token_data.claims.sub) {
                req.headers_mut().insert(USER_UUID_KEY, uuid_header);
            }

            // 4. Forward to inner service
            inner.call(req).await
        })
    }
}
