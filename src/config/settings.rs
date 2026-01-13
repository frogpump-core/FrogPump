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

    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir();
        fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self).map_err(|e| {
            FrogError::Config(format!("Failed to serialize config: {}", e))
        })?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    pub fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "api_base_url" => self.api_base_url = value.to_string(),
            "agent_id" => self.agent_id = Some(value.to_string()),
            "wallet_address" => self.wallet_address = Some(value.to_string()),
            "network" => {
                self.network = match value {
                    "mainnet" => Network::Mainnet,
                    "devnet" => Network::Devnet,
                    "localnet" => Network::Localnet,
                    _ => return Err(FrogError::Config(format!("Unknown network: {}", value))),
                };
                self.rpc_url = self.network.default_rpc().to_string();
            }
            "rpc_url" => self.rpc_url = value.to_string(),
            "verbose" => {
                self.verbose = value.parse().map_err(|_| {
                    FrogError::Config("verbose must be true or false".to_string())
                })?;
            }
            _ => return Err(FrogError::Config(format!("Unknown setting: {}", key))),
        }
        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        match key {
            "api_base_url" => Some(self.api_base_url.clone()),
            "agent_id" => self.agent_id.clone(),
            "wallet_address" => self.wallet_address.clone(),
            "network" => Some(format!("{:?}", self.network)),
            "rpc_url" => Some(self.rpc_url.clone()),
            "verbose" => Some(self.verbose.to_string()),
            _ => None,
        }
    }
}
