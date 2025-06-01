use anyhow::{Context, Result};
use bon::Builder;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const CONFIG_FILE_PATH: &str = "client_config.toml";

#[derive(Builder, Serialize, Deserialize, Debug, Clone)]
pub struct GrpcSettings {
    pub server_address: String,
    pub user_name: String,
    pub user_password: String,
}

impl Default for GrpcSettings {
    fn default() -> Self {
        GrpcSettings {
            server_address: "http://[::1]:50051".to_string(),
            user_name: "admin".to_string(),
            user_password: "my-secret-password".to_string(),
        }
    }
}

#[derive(Builder, Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClientConfig {
    pub grpc: GrpcSettings,
}

pub fn load_config() -> Result<ClientConfig> {
    let path = Path::new(CONFIG_FILE_PATH);
    if path.exists() {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file at {}", CONFIG_FILE_PATH))?;
        let config: ClientConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse TOML from {}", CONFIG_FILE_PATH))?;
        Ok(config)
    } else {
        println!(
            "Config file not found at {}, creating with default values.",
            CONFIG_FILE_PATH
        );
        let default_config = ClientConfig::default();
        save_config(&default_config)?;
        Ok(default_config)
    }
}

pub fn save_config(config: &ClientConfig) -> Result<()> {
    let toml_string =
        toml::to_string_pretty(config).context("Failed to serialize config to TOML")?;
    fs::write(CONFIG_FILE_PATH, toml_string)
        .with_context(|| format!("Failed to write config file to {}", CONFIG_FILE_PATH))?;
    println!("Configuration saved to {}", CONFIG_FILE_PATH);
    Ok(())
}
