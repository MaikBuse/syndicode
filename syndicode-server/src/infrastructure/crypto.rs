pub mod claims;
pub mod jwt;
pub mod password;

use std::sync::Arc;

use crate::config::ServerConfig;
use argon2::Argon2;
use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Clone)]
pub struct CryptoService {
    jwt_decoding_key: DecodingKey,
    jwt_encoding_key: EncodingKey,
    argon: Argon2<'static>,
}

impl CryptoService {
    pub fn new(config: Arc<ServerConfig>) -> anyhow::Result<Self> {
        let jwt_secret_bytes = config.auth.jwt_secret.as_bytes();

        Ok(Self {
            jwt_decoding_key: DecodingKey::from_secret(jwt_secret_bytes),
            jwt_encoding_key: EncodingKey::from_secret(jwt_secret_bytes),
            argon: Argon2::default(),
        })
    }
}
