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
            .with_context(|| format!("Failed to read config file at {}", path_str))?;
        toml::from_str::<ClientConfig>(&content)
            .with_context(|| format!("Failed to parse TOML from {}", path_str))?
    } else {
        println!(
            "Config file not found at {}, creating with default values.",
            path_str
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
        .with_context(|| format!("Failed to write config file to {}", path_str))?;
    println!("Configuration saved to {}", path_str);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_path_is_in_workspace_root() {
        // This test demonstrates that the path is correct.
        // It will fail if the build.rs script is not working.
        let config_path = get_config_path();
        let current_dir = env::current_dir().unwrap();

        println!("Current Directory (of test): {}", current_dir.display());
        println!("Resolved Config Path: {}", config_path.display());

        // The config path should NOT be in the current test directory
        // unless the crate IS the workspace root.
        let components = config_path.components();
        let config_parent = components.as_path();

        let mut cargo_toml = config_parent.to_path_buf();
        cargo_toml.push("Cargo.toml");

        assert!(
            cargo_toml.exists(),
            "Cargo.toml should exist in the parent of the config file path"
        );
        let content = fs::read_to_string(cargo_toml).unwrap();
        assert!(content.contains("[workspace]"), "The resolved path should be in the workspace root, which contains '[workspace]' in its Cargo.toml");

        // Cleanup: remove the config file if it was created
        if config_path.exists() {
            fs::remove_file(config_path).unwrap();
        }
    }

    #[test]
    fn test_load_and_save() {
        // Ensure we have a clean slate
        let config_path = get_config_path();
        if config_path.exists() {
            fs::remove_file(config_path).unwrap();
        }

        // 1. Should create a default config if none exists
        let config1 = load_config().unwrap();
        assert!(config_path.exists());
        assert_eq!(config1.grpc.server_address, "https://api.syndicode.dev");

        // 2. Should save and load correctly
        let mut new_config = ClientConfig::default();
        new_config.grpc.user_name = "test_user".to_string();
        save_config(&new_config).unwrap();

        let loaded_config = load_config().unwrap();
        assert_eq!(loaded_config.grpc.user_name, "test_user");

        // Cleanup
        fs::remove_file(config_path).unwrap();
    }
}
