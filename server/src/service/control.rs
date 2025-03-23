use crate::domain::{
    model::{
        control::{Claims, SessionModel, SessionState, SessionUser, UserModel, UserRole},
        economy::CorporationModel,
    },
    repository::{
        control::{ControlDatabaseError, ControlDatabaseRepository},
        economy::{EconomyDatabaseError, EconomyDatabaseRepository},
    },
};
use argon2::Argon2;
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::TryRngCore;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use uuid::Uuid;

const DEFAULT_BALANCE: i64 = 1000000;

#[derive(Debug, thiserror::Error)]
pub enum ControlServiceError {
    #[error("The game is already running")]
    SessionAlreadyRunning,

    #[error("The game is not running")]
    SessionNotRunning,

    #[error("The game is already initialized")]
    SessionAlreadyInitialized,

    #[error("Failed to turn slice of bytes into Uuid")]
    UuidFromSlice,

    #[error("The provided credentials are wrong")]
    WrongUserCredentials,

    #[error(transparent)]
    ControlDatabase(#[from] ControlDatabaseError),

    #[error(transparent)]
    EconomyDatabase(#[from] EconomyDatabaseError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ControlServiceResult<T> = std::result::Result<T, ControlServiceError>;

#[derive(Debug)]
pub struct ControlService {
    control_db: Arc<Mutex<dyn ControlDatabaseRepository>>,
    economy_db: Arc<Mutex<dyn EconomyDatabaseRepository>>,
    argon: Argon2<'static>,
    jwt_secret: String,
}

impl ControlService {
    pub fn new(
        control_db: Arc<Mutex<dyn ControlDatabaseRepository>>,
        economy_db: Arc<Mutex<dyn EconomyDatabaseRepository>>,
        jwt_secret: String,
    ) -> Self {
        Self {
            control_db,
            economy_db,
            jwt_secret,
            argon: Argon2::default(),
        }
    }

    pub async fn login(&self, username: String, password: String) -> ControlServiceResult<String> {
        let control_db = self.control_db.lock().await;
        let Ok(user) = control_db.get_user_by_name(username).await else {
            return Err(ControlServiceError::WrongUserCredentials);
        };
        let Ok(uuid) = Uuid::from_slice(&user.uuid) else {
            return Err(ControlServiceError::UuidFromSlice);
        };

        let mut password_hashed = Vec::<u8>::new();
        if let Err(err) =
            self.argon
                .hash_password_into(password.as_bytes(), &user.salt, &mut password_hashed)
        {
            return Err(anyhow::anyhow!("Failed to hash password: {}", err).into());
        }

        if password_hashed != user.password_hash {
            return Err(ControlServiceError::WrongUserCredentials);
        }

        let expiration = SystemTime::now()
            .checked_add(Duration::from_secs(86400))
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            sub: uuid.to_string(),
            exp: expiration,
            role: user.role,
        };

        let jwt = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        ) {
            Ok(jwt) => jwt,
            Err(err) => {
                return Err(anyhow::anyhow!("{}", err).into());
            }
        };

        Ok(jwt)
    }

    pub async fn create_session(&self) -> ControlServiceResult<SessionModel> {
        let control_db = self.control_db.lock().await;

        let session = control_db.create_session(Uuid::now_v7().into()).await?;

        Ok(session)
    }

    pub async fn get_session(&self, session_uuid: Vec<u8>) -> ControlServiceResult<SessionModel> {
        let control_db = self.control_db.lock().await;

        let session = control_db.get_session(session_uuid).await?;

        Ok(session)
    }

    pub async fn list_sessions(&self) -> ControlServiceResult<Vec<SessionModel>> {
        let control_db = self.control_db.lock().await;

        let session = control_db.list_sessions().await?;

        Ok(session)
    }

    pub async fn update_session_state(
        &self,
        session_uuid: Vec<u8>,
        req_session_state: SessionState,
    ) -> ControlServiceResult<()> {
        let control_db = self.control_db.lock().await;

        let curr_session = control_db.get_session(session_uuid).await?;

        let curr_session_state = SessionState::try_from(curr_session.state)?;

        match req_session_state {
            SessionState::Initializing => match curr_session_state {
                SessionState::Initializing => {
                    return Err(ControlServiceError::SessionAlreadyInitialized);
                }
                _ => {}
            },
            SessionState::Running => match curr_session_state {
                SessionState::Running => {
                    return Err(ControlServiceError::SessionAlreadyRunning);
                }
                _ => {}
            },
        }

        control_db
            .update_session(SessionModel {
                uuid: curr_session.uuid,
                interval: curr_session.interval,
                state: req_session_state.to_string(),
            })
            .await?;

        Ok(())
    }

    pub async fn advance_session_interval(
        &self,
        session_uuid: Vec<u8>,
    ) -> ControlServiceResult<()> {
        let control_db = self.control_db.lock().await;

        let mut session = control_db.get_session(session_uuid).await?;

        session.interval += 1;

        control_db.update_session(session).await?;

        Ok(())
    }

    pub async fn delete_session(&self, session_uuid: Vec<u8>) -> ControlServiceResult<()> {
        let control_db = self.control_db.lock().await;

        Ok(control_db.delete_session(session_uuid).await?)
    }

    pub async fn join_game(
        &self,
        session_uuid: Vec<u8>,
        user_uuid: Vec<u8>,
        corporation_name: String,
    ) -> ControlServiceResult<CorporationModel> {
        let control_db = self.control_db.lock().await;

        let session_user = SessionUser {
            uuid: Uuid::now_v7().into(),
            session_uuid: session_uuid.clone(),
            user_uuid: session_uuid.clone(),
        };

        control_db.create_session_user(session_user).await?;

        let economy_db = self.economy_db.lock().await;

        let corporation = CorporationModel {
            uuid: Uuid::now_v7().into(),
            session_uuid,
            user_uuid,
            name: corporation_name,
            balance: DEFAULT_BALANCE,
        };

        Ok(economy_db.create_corporation(corporation).await?)
    }

    pub async fn create_user(
        &self,
        username: String,
        password: Vec<u8>,
        user_role: UserRole,
    ) -> ControlServiceResult<UserModel> {
        let mut salt = [0u8; 16];
        if let Err(err) = rand::rng().try_fill_bytes(&mut salt) {
            return Err(anyhow::anyhow!("Failed to generate salt: {}", err).into());
        }

        let mut password_hashed = Vec::<u8>::new();
        if let Err(err) = self
            .argon
            .hash_password_into(&password, &salt, &mut password_hashed)
        {
            return Err(anyhow::anyhow!("Failed to hash password: {}", err).into());
        }

        let user = UserModel {
            uuid: Uuid::now_v7().into(),
            name: username,
            password_hash: password_hashed,
            salt: salt.into(),
            role: user_role.to_string(),
        };

        let control_db = self.control_db.lock().await;

        Ok(control_db.create_user(user).await?)
    }

    pub async fn get_user(&self, user_uuid: Vec<u8>) -> ControlServiceResult<UserModel> {
        let control_db = self.control_db.lock().await;

        Ok(control_db.get_user(user_uuid).await?)
    }
}
