use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const CONFIG_FILE_PATH: &str = "app_config.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GrpcSettings {
    pub auth_service_address: String,
}

impl Default for GrpcSettings {
    fn default() -> Self {
        GrpcSettings {
            auth_service_address: "http://[::1]:50051".to_string(), // Sensible default
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppConfig {
    pub grpc: GrpcSettings,
}

pub fn load_config() -> Result<AppConfig> {
    let path = Path::new(CONFIG_FILE_PATH);
    if path.exists() {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file at {}", CONFIG_FILE_PATH))?;
        let config: AppConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse TOML from {}", CONFIG_FILE_PATH))?;
        Ok(config)
    } else {
        println!(
            "Config file not found at {}, creating with default values.",
            CONFIG_FILE_PATH
        );
        let default_config = AppConfig::default();
        save_config(&default_config)?;
        Ok(default_config)
    }
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let toml_string =
        toml::to_string_pretty(config).context("Failed to serialize config to TOML")?;
    fs::write(CONFIG_FILE_PATH, toml_string)
        .with_context(|| format!("Failed to write config file to {}", CONFIG_FILE_PATH))?;
    println!("Configuration saved to {}", CONFIG_FILE_PATH);
    Ok(())
}
