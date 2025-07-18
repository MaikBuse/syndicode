use anyhow::Context;
use bon::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{application::ports::limiter::LimiterCategory, utils::read_env_var};
use std::path::Path;

pub const CONFIG_FILE_PATH: &str = "server_config.toml";

#[derive(Builder, Serialize, Deserialize, Debug, Clone)]
pub struct GeneralConfig {
    /// A unique identifier for this specific instance trying to acquire the lock.
    /// Used to ensure only the lock holder can release/refresh it.
    pub instance_id: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            instance_id: Uuid::now_v7().to_string(),
        }
    }
}

#[derive(Builder, Serialize, Deserialize, Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub admin_email: String,
    pub admin_username: String,
    pub admin_password: String,
    pub admin_corporation_name: String,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "super-secret-jwt".to_string(),
            admin_email: "contact@maikbuse.com".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "super-secret-password".to_string(),
            admin_corporation_name: "Shinkai Heavyworks".to_string(),
        }
    }
}

#[derive(Builder, Serialize, Deserialize, Debug, Clone)]
pub struct BootstrapConfig {
    pub business_count_x: usize,
    pub business_count_y: usize,
    pub boundary_min_lon: f64,
    pub boundary_max_lon: f64,
    pub boundary_min_lat: f64,
    pub boundary_max_lat: f64,
    /// Business clusters have a characteristic spread
    pub spread_sigma_meters: f64,
    pub max_radius_meters: f64,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        Self {
            business_count_x: 10,
            business_count_y: 10,
            boundary_min_lon: 139.6,
            boundary_max_lon: 139.9,
            boundary_min_lat: 35.5,
            boundary_max_lat: 35.8,
            spread_sigma_meters: 200.0,
            max_radius_meters: 800.0,
        }
    }
}

#[derive(Builder, Serialize, Deserialize, Debug, Clone)]
pub struct RateLimiterConfig {
    pub disable_rate_limiting: bool,
    pub ip_address_header: String,
    pub proxy_api_key: String,
    pub middleware_max_req: usize,
    pub middleware_window_secs: usize,
    pub game_stream_max_req: usize,
    pub game_stream_window_secs: usize,
    pub auth_max_req: usize,
    pub auth_window_secs: usize,
    pub admin_max_req: usize,
    pub admin_window_secs: usize,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            disable_rate_limiting: false,
            ip_address_header: "CF-Connecting-IP".to_string(),
            proxy_api_key: "super-secret-api-key".to_string(),
            middleware_max_req: 150,
            middleware_window_secs: 60,
            game_stream_max_req: 100,
            game_stream_window_secs: 10,
            auth_max_req: 5,
            auth_window_secs: 60,
            admin_max_req: 10,
            admin_window_secs: 60,
        }
    }
}

impl RateLimiterConfig {
    pub fn get_max_requests(&self, category: LimiterCategory) -> usize {
        match category {
            LimiterCategory::Middleware => self.middleware_max_req,
            LimiterCategory::Game => self.game_stream_max_req,
            LimiterCategory::Auth => self.auth_max_req,
            LimiterCategory::Admin => self.admin_max_req,
        }
    }

    pub fn get_window_secs(&self, category: LimiterCategory) -> usize {
        match category {
            LimiterCategory::Middleware => self.middleware_window_secs,
            LimiterCategory::Game => self.game_stream_window_secs,
            LimiterCategory::Auth => self.auth_window_secs,
            LimiterCategory::Admin => self.admin_window_secs,
        }
    }
}

#[derive(Builder, Serialize, Deserialize, Debug, Clone)]
pub struct ProcessorConfig {
    pub game_tick_interval: usize,
    pub leader_lock_ttl: usize,
    pub leader_lock_refresh_interval: usize,
    pub non_leader_acquisition_retry_internal: usize,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            game_tick_interval: 3000,
            leader_lock_ttl: 90000,
            leader_lock_refresh_interval: 30000,
            non_leader_acquisition_retry_internal: 15000,
        }
    }
}

#[derive(Builder, Serialize, Deserialize, Debug, Clone)]
pub struct PostgresConfig {
    pub max_connections: u32,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: usize,
    pub database: String,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            max_connections: 5,
            user: "postgres".to_string(),
            password: "secretpassword".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            database: "syndicode".to_string(),
        }
    }
}

#[derive(Builder, Serialize, Deserialize, Debug, Clone)]
pub struct ValkeyConfig {
    pub host: String,
    pub password: String,
    /// How many messages to request per internal batch
    pub batch_pull_size: usize,
}

impl Default for ValkeyConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            password: "secretpassword".to_string(),
            batch_pull_size: 100,
        }
    }
}

#[derive(Builder, Serialize, Deserialize, Debug, Clone)]
pub struct EmailConfig {
    pub sender_email: String,
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl Default for EmailConfig {
    fn default() -> Self {
        let smtp_server = read_env_var("SERVER_SMTP_SERVER").unwrap();
        let smtp_username = read_env_var("SERVER_SMTP_USERNAME").unwrap();
        let smtp_password = read_env_var("SERVER_SMTP_PASSWORD").unwrap();

        Self {
            sender_email: "noreply@syndicode.dev".to_string(),
            smtp_server,
            smtp_username,
            smtp_password,
        }
    }
}

#[derive(Builder, Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServerConfig {
    pub general: GeneralConfig,
    pub auth: AuthConfig,
    pub bootstrap: BootstrapConfig,
    pub rate_limiter: RateLimiterConfig,
    pub processor: ProcessorConfig,
    pub postgres: PostgresConfig,
    pub valkey: ValkeyConfig,
    pub email: EmailConfig,
}

impl ServerConfig {
    pub fn new() -> anyhow::Result<Self> {
        let path = Path::new(CONFIG_FILE_PATH);

        let mut config = if path.exists() {
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("Failed to read config file at {CONFIG_FILE_PATH}"))?;
            toml::from_str::<ServerConfig>(&content)
                .with_context(|| format!("Failed to parse TOML from {CONFIG_FILE_PATH}"))?
        } else {
            tracing::info!(
                "Config file not found at {CONFIG_FILE_PATH}, creating with default values."
            );
            let default_config = ServerConfig::default();
            save_config(&default_config)?;
            default_config
        };

        // GeneralConfig
        if let Ok(val) = read_env_var("SERVER_INSTANCE_ID") {
            config.general.instance_id = val;
        }

        // AuthConfig
        if let Ok(val) = read_env_var("SERVER_JWT_SECRET") {
            config.auth.jwt_secret = val;
        }
        if let Ok(val) = read_env_var("SERVER_ADMIN_EMAIL") {
            config.auth.admin_email = val;
        }
        if let Ok(val) = read_env_var("SERVER_ADMIN_USERNAME") {
            config.auth.admin_email = val;
        }
        if let Ok(val) = read_env_var("SERVER_ADMIN_PASSWORD") {
            config.auth.admin_password = val;
        }
        if let Ok(val) = read_env_var("SERVER_ADMIN_CORPORATION_NAME") {
            config.auth.admin_corporation_name = val;
        }

        // BootstrapConfig
        if let Ok(val) = int_from_env("SERVER_BUSINESS_COUNT_X") {
            config.bootstrap.business_count_x = val;
        }
        if let Ok(val) = int_from_env("SERVER_BUSINESS_COUNT_Y") {
            config.bootstrap.business_count_y = val;
        }
        if let Ok(val) = read_env_var("SERVER_BOUNDARY_MIN_LON") {
            if let Ok(parsed) = val.parse() {
                config.bootstrap.boundary_min_lon = parsed;
            }
        }
        if let Ok(val) = read_env_var("SERVER_BOUNDARY_MAX_LON") {
            if let Ok(parsed) = val.parse() {
                config.bootstrap.boundary_max_lon = parsed;
            }
        }
        if let Ok(val) = read_env_var("SERVER_BOUNDARY_MIN_LAT") {
            if let Ok(parsed) = val.parse() {
                config.bootstrap.boundary_min_lat = parsed;
            }
        }
        if let Ok(val) = read_env_var("SERVER_BOUNDARY_MAX_LAT") {
            if let Ok(parsed) = val.parse() {
                config.bootstrap.boundary_max_lat = parsed;
            }
        }
        if let Ok(val) = read_env_var("SERVER_SPREAD_SIGMA_METERS") {
            if let Ok(parsed) = val.parse() {
                config.bootstrap.spread_sigma_meters = parsed;
            }
        }
        if let Ok(val) = read_env_var("SERVER_MAX_RADIUS_METERS") {
            if let Ok(parsed) = val.parse() {
                config.bootstrap.max_radius_meters = parsed;
            }
        }

        // RateLimiterConfig
        if let Ok(val) = read_env_var("SERVER_DISABLE_RATE_LIMITING") {
            if let Ok(parsed) = val.parse() {
                config.rate_limiter.disable_rate_limiting = parsed;
            }
        }
        if let Ok(val) = read_env_var("SERVER_IP_ADDRESS_HEADER") {
            config.rate_limiter.ip_address_header = val;
        }
        if let Ok(val) = read_env_var("SERVER_PROXY_API_KEY") {
            config.rate_limiter.proxy_api_key = val;
        }
        if let Ok(val) = int_from_env("SERVER_MIDDLEWARE_MAX_REQ") {
            config.rate_limiter.middleware_max_req = val;
        }
        if let Ok(val) = int_from_env("SERVER_MIDDLEWARE_WINDOW_SECS") {
            config.rate_limiter.middleware_window_secs = val;
        }
        if let Ok(val) = int_from_env("SERVER_GAME_STREAM_MAX_REQ") {
            config.rate_limiter.game_stream_max_req = val;
        }
        if let Ok(val) = int_from_env("SERVER_GAME_STREAM_WINDOW_SECS") {
            config.rate_limiter.game_stream_window_secs = val;
        }
        if let Ok(val) = int_from_env("SERVER_AUTH_MAX_REQ") {
            config.rate_limiter.auth_max_req = val;
        }
        if let Ok(val) = int_from_env("SERVER_AUTH_WINDOW_SECS") {
            config.rate_limiter.auth_window_secs = val;
        }
        if let Ok(val) = int_from_env("SERVER_ADMIN_MAX_REQ") {
            config.rate_limiter.admin_max_req = val;
        }
        if let Ok(val) = int_from_env("SERVER_ADMIN_WINDOW_SECS") {
            config.rate_limiter.admin_window_secs = val;
        }

        // ProcessorConfig
        if let Ok(val) = int_from_env("SERVER_GAME_TICK_INTERVAL") {
            config.processor.game_tick_interval = val;
        }
        if let Ok(val) = int_from_env("SERVER_LEADER_LOCK_TTL") {
            config.processor.leader_lock_ttl = val;
        }
        if let Ok(val) = int_from_env("SERVER_LEADER_LOCK_REFRESH_INTERVAL") {
            config.processor.leader_lock_refresh_interval = val;
        }
        if let Ok(val) = int_from_env("SERVER_NON_LEADER_ACQUISITION_RETRY_INTERNAL") {
            config.processor.non_leader_acquisition_retry_internal = val;
        }

        // PostgresConfig
        if let Ok(val) = int_from_env("SERVER_POSTGRES_MAX_CONNECTIONS") {
            config.postgres.max_connections = val;
        }
        if let Ok(val) = read_env_var("SERVER_POSTGRES_USER") {
            config.postgres.user = val;
        }
        if let Ok(val) = read_env_var("SERVER_POSTGRES_PASSWORD") {
            config.postgres.password = val;
        }
        if let Ok(val) = read_env_var("SERVER_POSTGRES_HOST") {
            config.postgres.host = val;
        }
        if let Ok(val) = int_from_env("SERVER_POSTGRES_PORT") {
            config.postgres.port = val;
        }
        if let Ok(val) = read_env_var("SERVER_POSTGRES_DATABASE") {
            config.postgres.database = val;
        }

        // ValkeyConfig
        if let Ok(val) = read_env_var("SERVER_VALKEY_HOST") {
            config.valkey.host = val;
        }
        if let Ok(val) = read_env_var("SERVER_VALKEY_PASSWORD") {
            config.valkey.password = val;
        }
        if let Ok(val) = int_from_env("SERVER_VALKEY_BATCH_PULL_SIZE") {
            config.valkey.batch_pull_size = val;
        }

        // EmailConfig
        if let Ok(val) = read_env_var("SERVER_SENDER_EMAIL") {
            config.email.sender_email = val;
        }
        if let Ok(val) = read_env_var("SERVER_SMTP_USERNAME") {
            config.email.smtp_username = val;
        }
        if let Ok(val) = read_env_var("SERVER_SMTP_PASSWORD") {
            config.email.smtp_password = val;
        }
        if let Ok(val) = read_env_var("SERVER_SMTP_SERVER") {
            config.email.smtp_server = val;
        }

        Ok(config)
    }
}

pub fn save_config(config: &ServerConfig) -> anyhow::Result<()> {
    let toml_string =
        toml::to_string_pretty(config).context("Failed to serialize config to TOML")?;
    std::fs::write(CONFIG_FILE_PATH, toml_string)
        .with_context(|| format!("Failed to write config file to {CONFIG_FILE_PATH}"))?;
    tracing::info!("Configuration saved to {CONFIG_FILE_PATH}");
    Ok(())
}

fn int_from_env<U>(env_name: &str) -> anyhow::Result<U>
where
    U: std::str::FromStr,
{
    let value_string = read_env_var(env_name)?;
    let Ok(value_int) = value_string.parse::<U>() else {
        return Err(anyhow::anyhow!("Failed to parse usize from '{}'", env_name));
    };
    Ok(value_int)
}
