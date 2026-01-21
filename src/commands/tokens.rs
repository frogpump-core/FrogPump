use anyhow::{Context, Result};

use crate::api::client::ApiClient;
use crate::cli::output::OutputFormatter;
use crate::cli::parser::validate_agent_id;
use crate::config::settings::Settings;
use crate::utils::display;

pub async fn execute(agent_id: Option<String>, verbose: bool, config: &Settings) -> Result<()> {
    let agent_id = match agent_id.or_else(|| config.agent_id.clone()) {
        Some(id) => {
            validate_agent_id(&id).map_err(|e| anyhow::anyhow!("{}", e))?;
            id
        }
        None => anyhow::bail!("Agent ID required. Pass --agent-id or set it in config."),
    };

    let api = ApiClient::new(&config.api_base_url);
    let tokens = api
        .get_tokens(&agent_id)
        .await
        .context("Failed to fetch tokens")?;

    display::print_header(&format!("Tokens for agent: {}", agent_id));

