use anyhow::{Context, Result};
use bon::Builder;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Builder, Serialize, Deserialize, Debug, Clone, Default)]
pub struct GeneralSettings {
    pub is_local_test: bool,
}

#[derive(Builder, Serialize, Deserialize, Debug, Clone)]
pub struct GrpcSettings {
    pub server_address: String,
    pub user_name: String,
    pub user_password: String,
}

impl Default for GrpcSettings {
    fn default() -> Self {
        GrpcSettings {
            server_address: "https://api.syndicode.dev".to_string(),
            user_name: "".to_string(),
            user_password: "".to_string(),
        }
    }
}

#[derive(Builder, Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClientConfig {
    pub general: GeneralSettings,
    pub grpc: GrpcSettings,
}

// This computes the path once and reuses it.
static CONFIG_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    // This path is baked into the binary.
    let workspace_root = env!("WORKSPACE_ROOT");
    Path::new(workspace_root).join("client_config.toml")
});

/// Returns the absolute path to the configuration file.
pub fn get_config_path() -> &'static Path {
    &CONFIG_FILE_PATH
}

pub fn load_config() -> Result<ClientConfig> {
    use std::env;

    let path = get_config_path();
    let path_str = path.display().to_string();

    let mut config = if path.exists() {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file at {path_str}"))?;
        toml::from_str::<ClientConfig>(&content)
            .with_context(|| format!("Failed to parse TOML from {path_str}"))?
    } else {
        println!(
            "Config file not found at {path_str}, creating with default values."
        );
        let default_config = ClientConfig::default();
        save_config(&default_config)?;
        default_config
    };

    // Override with env vars if present
    if let Ok(is_local_test_string) = env::var("SYNDICODE_IS_LOCAL_TEST") {
        if let Ok(is_local_test) = is_local_test_string.parse::<bool>() {
            config.general.is_local_test = is_local_test;
        }
    }
    if let Ok(addr) = env::var("SYNDICODE_SERVER_ADDRESS") {
        config.grpc.server_address = addr;
    }
    if let Ok(user) = env::var("SYNDICODE_USER_NAME") {
        config.grpc.user_name = user;
    }
    if let Ok(pass) = env::var("SYNDICODE_USER_PASSWORD") {
        config.grpc.user_password = pass;
    }

    Ok(config)
}

pub fn save_config(config: &ClientConfig) -> Result<()> {
    let path = get_config_path();
    let path_str = path.display().to_string();

    let toml_string =
        toml::to_string_pretty(config).context("Failed to serialize config to TOML")?;
    fs::write(path, toml_string)
        .with_context(|| format!("Failed to write config file to {path_str}"))?;
    println!("Configuration saved to {path_str}");
    Ok(())
}
