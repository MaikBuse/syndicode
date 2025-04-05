use super::{claims::Claims, CryptoService};
use crate::{application::ports::crypto::JwtHandler, domain::user::model::role::UserRole};
use jsonwebtoken::TokenData;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const VALID_DURATION: Duration = Duration::from_secs(86400);

impl JwtHandler for CryptoService {
    fn decode_jwt(&self, token: &str) -> anyhow::Result<TokenData<Claims>> {
        decode::<Claims>(
            token,
            &self.jwt_decoding_key,
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| anyhow::anyhow!("Invalid or expired token"))
    }

    fn encode_jwt(&self, user_uuid: Uuid, user_role: UserRole) -> anyhow::Result<String> {
        let expiration = SystemTime::now()
            .checked_add(VALID_DURATION)
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            sub: user_uuid.to_string(),
            exp: expiration,
            role: user_role.to_string(),
        };

        match encode(&Header::default(), &claims, &self.jwt_encoding_key) {
            Ok(jwt) => Ok(jwt),
            Err(err) => Err(anyhow::anyhow!("{}", err.to_string())),
        }
    }
}
