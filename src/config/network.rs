use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Devnet,
    Localnet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub name: String,
    pub rpc_url: String,
    pub ws_url: String,
    pub explorer_url: String,
}

impl Network {
    pub fn config(&self) -> NetworkConfig {
        match self {
            Network::Mainnet => NetworkConfig {
                name: "mainnet-beta".to_string(),
                rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
                ws_url: "wss://api.mainnet-beta.solana.com".to_string(),
                explorer_url: "https://explorer.solana.com".to_string(),
            },
            Network::Devnet => NetworkConfig {
                name: "devnet".to_string(),
                rpc_url: "https://api.devnet.solana.com".to_string(),
                ws_url: "wss://api.devnet.solana.com".to_string(),
                explorer_url: "https://explorer.solana.com/?cluster=devnet".to_string(),
            },
            Network::Localnet => NetworkConfig {
                name: "localnet".to_string(),
                rpc_url: "http://localhost:8899".to_string(),
                ws_url: "ws://localhost:8900".to_string(),
                explorer_url: "https://explorer.solana.com/?cluster=custom".to_string(),
            },
        }
    }

    pub fn default_rpc(&self) -> &str {
        match self {
            Network::Mainnet => "https://api.mainnet-beta.solana.com",
            Network::Devnet => "https://api.devnet.solana.com",
            Network::Localnet => "http://localhost:8899",
        }
    }
}

// iteration 72
