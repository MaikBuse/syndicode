use super::{claims::Claims, CryptoService};
use crate::application::ports::crypto::JwtHandler;
use crate::domain::user::model::User;
use jsonwebtoken::TokenData;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

    fn encode_jwt(&self, user: &User) -> anyhow::Result<String> {
        let expiration = SystemTime::now()
            .checked_add(VALID_DURATION)
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            sub: user.uuid.to_string(),
            exp: expiration,
            user_name: user.name.to_string(),
            user_role: user.role.to_string(),
            user_email: user.email.to_string(),
        };

        match encode(&Header::default(), &claims, &self.jwt_encoding_key) {
            Ok(jwt) => Ok(jwt),
            Err(err) => Err(anyhow::anyhow!("{}", err.to_string())),
        }
    }
}
