use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use std::time::Duration;

use crate::api::client::ApiClient;
use crate::cli::app::LaunchArgs;
use crate::cli::output::OutputFormatter;
use crate::cli::parser::validate_agent_id;
use crate::config::settings::Settings;
use crate::core::engine::LaunchEngine;
use crate::utils::display;

pub async fn execute(args: LaunchArgs, config: &Settings) -> Result<()> {
    let agent_id = match args.agent_id.clone().or_else(|| config.agent_id.clone()) {
        Some(id) => {
            validate_agent_id(&id).map_err(|e| anyhow::anyhow!("{}", e))?;
            id
        }
        None => anyhow::bail!("Agent ID required. Pass --agent-id or set it in config."),
    };

    info!(
        "Launching token {} ({}) for agent {}",
        args.name, args.symbol, agent_id
    );

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("valid template"),
    );
    spinner.set_message(format!("Launching {} ({})...", args.name, args.symbol));
    spinner.enable_steady_tick(Duration::from_millis(100));

