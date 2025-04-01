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
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_decoding_key: DecodingKey::from_secret(jwt_secret.as_bytes()),
            jwt_encoding_key: EncodingKey::from_secret(jwt_secret.as_bytes()),
            argon: Argon2::default(),
        }
    }
}
