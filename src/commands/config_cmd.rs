use anyhow::Result;
use colored::Colorize;

use crate::config::settings::Settings;
use crate::utils::display;

pub fn execute_set(key: &str, value: &str) -> Result<()> {
    let mut settings = Settings::load().unwrap_or_default();

    settings
        .set_value(key, value)
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    settings.save().map_err(|e| anyhow::anyhow!("{}", e))?;

    println!("{} {} = {}", "[OK]".green().bold(), key.cyan(), value);

    Ok(())
}

pub fn execute_get(key: &str) -> Result<()> {
    let settings = Settings::load().unwrap_or_default();

    match settings.get_value(key) {
        Some(value) => {
            println!("  {} = {}", key.cyan(), value);
        }
        None => {
            anyhow::bail!(
                "Unknown config key '{}'. Valid keys: api_base_url, agent_id, wallet_address, network, rpc_url, verbose",
                key
            );
        }
    }

