use std::env;

pub fn read_env_var(key: &str) -> anyhow::Result<String> {
    env::var(key).map_err(|_| anyhow::anyhow!("Environment variable '{}' must be set", key))
}
