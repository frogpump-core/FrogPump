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

