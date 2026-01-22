use anyhow::{Context, Result};
use std::io::{self, Write};

use crate::api::client::ApiClient;
use crate::cli::app::ClaimArgs;
use crate::cli::output::OutputFormatter;
use crate::cli::parser::validate_agent_id;
use crate::config::settings::Settings;
use crate::core::fee_collector::FeeCollector;
use crate::utils::display;

pub async fn execute(args: ClaimArgs, config: &Settings) -> Result<()> {
    let agent_id = match args.agent_id.or_else(|| config.agent_id.clone()) {
        Some(id) => {
            validate_agent_id(&id).map_err(|e| anyhow::anyhow!("{}", e))?;
            id
        }
        None => anyhow::bail!("Agent ID required. Pass --agent-id or set it in config."),
    };

    let api = ApiClient::new(&config.api_base_url);
    let collector = FeeCollector::new(ApiClient::new(&config.api_base_url));

    // Fetch current unclaimed earnings
    let unclaimed = collector
        .get_unclaimed(&agent_id)
        .await
        .context("Failed to fetch earnings")?;

    if unclaimed.is_empty() {
        println!("  No unclaimed earnings for agent {}.", agent_id);
        return Ok(());
    }

