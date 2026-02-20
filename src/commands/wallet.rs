use anyhow::{Context, Result};

use crate::api::client::ApiClient;
use crate::api::types::WalletRequest;
use crate::cli::output::OutputFormatter;
use crate::cli::parser::{validate_address, validate_agent_id};
use crate::config::settings::Settings;
use crate::utils::display;

pub async fn execute_set(address: String, signature: String, config: &Settings) -> Result<()> {
    let address = validate_address(&address).map_err(|e| anyhow::anyhow!("{}", e))?;

    let agent_id = match config.agent_id.as_ref() {
        Some(id) => {
            validate_agent_id(id).map_err(|e| anyhow::anyhow!("{}", e))?;
            id.clone()
        }
        None => anyhow::bail!(
            "Agent ID required. Set it in config first: frogpump config set agent_id <id>"
        ),
    };

    let api = ApiClient::new(&config.api_base_url);
    let request = WalletRequest {
        agent_id: agent_id.clone(),
        wallet_address: address.clone(),
        signature,
        message: format!("frogpump:verify:{}", agent_id),
    };

    api.set_wallet(request)
        .await
        .context("Failed to set wallet")?;

    OutputFormatter::print_success(&format!(
        "Wallet {} associated with agent {}",
        display::short_address(&address),
        agent_id
    ));

    Ok(())
}

pub async fn execute_show(agent_id: Option<String>, config: &Settings) -> Result<()> {
    let agent_id = match agent_id.or_else(|| config.agent_id.clone()) {
        Some(id) => {
            validate_agent_id(&id).map_err(|e| anyhow::anyhow!("{}", e))?;
            id
        }
        None => anyhow::bail!("Agent ID required. Pass --agent-id or set it in config."),
    };

    let wallet = config
        .wallet_address
        .as_deref()
        .unwrap_or("(not set)");

    display::print_header("Wallet Info");
    display::print_key_value("Agent", &agent_id);
    display::print_key_value("Wallet", wallet);

    Ok(())
}

// iteration 56
