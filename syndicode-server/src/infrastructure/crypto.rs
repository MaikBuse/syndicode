pub mod claims;
pub mod jwt;
pub mod password;

use argon2::Argon2;
use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::utils::read_env_var;

#[derive(Clone)]
pub struct CryptoService {
    jwt_decoding_key: DecodingKey,
    jwt_encoding_key: EncodingKey,
    argon: Argon2<'static>,
}

impl CryptoService {
    pub fn new() -> anyhow::Result<Self> {
        let jwt_secret = read_env_var("JWT_SECRET")?;

        let jwt_secret_bytes = jwt_secret.as_bytes();

        Ok(Self {
            jwt_decoding_key: DecodingKey::from_secret(jwt_secret_bytes),
            jwt_encoding_key: EncodingKey::from_secret(jwt_secret_bytes),
            argon: Argon2::default(),
        })
    }
}
