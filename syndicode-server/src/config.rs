use uuid::Uuid;

use crate::utils::read_env_var;
use std::time::Duration;

pub struct Config {
    /// A unique identifier for this specific instance trying to acquire the lock.
    /// Used to ensure only the lock holder can release/refresh it.
    pub instance_id: String,
    pub ip_address_header: String,
    pub game_tick_interval: usize,
    pub leader_lock_ttl: usize,
    pub leader_lock_refresh_interval: Duration,
    pub non_leader_acquisition_retry_internal: Duration,
    pub disable_rate_limitting: bool,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let instance_id = Uuid::now_v7().to_string();

        let ip_address_header = read_env_var("IP_ADDRESS_HEADER")?;

        let game_tick_interval = int_from_env("GAME_TICK_INTERVAL")?;
        let leader_lock_ttl = int_from_env("LEADER_LOCK_TTL")?;
        let leader_lock_refresh_interval = int_from_env("LEADER_LOCK_REFRESH")?;
        let non_leader_acquisition_retry_internal = int_from_env("NON_LEADER_RETRY")?;

        let disable_rate_limitting = read_env_var("DISABLE_RATE_LIMITING")?.parse::<bool>()?;

        Ok(Self {
            instance_id,
            ip_address_header,
            game_tick_interval,
            leader_lock_refresh_interval: Duration::from_millis(leader_lock_refresh_interval),
            leader_lock_ttl,
            non_leader_acquisition_retry_internal: Duration::from_millis(
                non_leader_acquisition_retry_internal,
            ),
            disable_rate_limitting,
        })
    }
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
