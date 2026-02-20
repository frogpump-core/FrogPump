pub mod cli;
pub mod commands;
pub mod core;
pub mod api;
pub mod solana;
pub mod models;
pub mod config;
pub mod utils;

pub use config::settings::Settings;
pub use config::network::Network;
pub use api::client::ApiClient;
pub use core::engine::LaunchEngine;
pub use utils::error::FrogError;

pub const APP_NAME: &str = "frogpump";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_RPC_URL: &str = "https://api.mainnet-beta.solana.com";
pub const DEFAULT_API_BASE: &str = "https://api.frogpump.fun/v1";
pub const CONFIG_FILENAME: &str = "config.toml";

pub fn config_dir() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(APP_NAME)
}
// chore: update version constants
