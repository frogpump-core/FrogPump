use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::network::Network;
use crate::utils::error::{FrogError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub api_base_url: String,
    pub agent_id: Option<String>,
    pub wallet_address: Option<String>,
    pub network: Network,
    pub rpc_url: String,
    pub verbose: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_base_url: "https://api.frogpump.com".to_string(),
            agent_id: None,
            wallet_address: None,
            network: Network::Mainnet,
            rpc_url: Network::Mainnet.default_rpc().to_string(),
            verbose: false,
        }
    }
}

impl Settings {
    pub fn config_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".frogpump")
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_dir().join("config.toml");
        if config_path.exists() {
            let content = fs::read_to_string(&config_path).map_err(|e| {
                FrogError::Config(format!("Failed to read config file: {}", e))
            })?;
            let settings: Settings = toml::from_str(&content).map_err(|e| {
                FrogError::Config(format!("Failed to parse config file: {}", e))
            })?;
            Ok(settings)
        } else {
            Ok(Self::default())
        }
    }

