use prost_types::Timestamp;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn read_env_var(key: &str) -> anyhow::Result<String> {
    env::var(key).map_err(|_| anyhow::anyhow!("Environment variable '{}' must be set", key))
}

pub fn timestamp_now() -> Result<Timestamp, &'static str> {
    let now = SystemTime::now();
    let duration_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "SystemTime is before UNIX_EPOCH")?;

    Ok(Timestamp {
        seconds: duration_since_epoch.as_secs() as i64,
        nanos: duration_since_epoch.subsec_nanos() as i32,
    })
}
