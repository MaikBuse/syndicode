pub mod claims;
pub mod jwt;
pub mod password;

use argon2::Argon2;
use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct CryptoService {
    jwt_decoding_key: DecodingKey,
    jwt_encoding_key: EncodingKey,
    argon: Argon2<'static>,
}

impl CryptoService {
    pub fn new() -> Self {
        let jwt_secret =
            std::env::var("JWT_SECRET").expect("Environment variable 'JWT_SECRET' must be set");

        let jwt_secret_bytes = jwt_secret.as_bytes();

        Self {
            jwt_decoding_key: DecodingKey::from_secret(jwt_secret_bytes),
            jwt_encoding_key: EncodingKey::from_secret(jwt_secret_bytes),
            argon: Argon2::default(),
        }
    }
}
