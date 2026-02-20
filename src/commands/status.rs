use anyhow::{Context, Result};

use crate::api::client::ApiClient;
use crate::cli::output::OutputFormatter;
use crate::cli::parser::validate_agent_id;
use crate::config::settings::Settings;
use crate::core::fee_collector::FeeCollector;
use crate::utils::display;

pub async fn execute(agent_id: Option<String>, config: &Settings) -> Result<()> {
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

    let earnings_resp = api
        .get_earnings(&agent_id)
        .await
        .context("Failed to fetch earnings")?;

    display::print_header(&format!("Status for agent: {}", agent_id));
    display::print_key_value("Tokens launched", &tokens.len().to_string());
    display::print_key_value("Total earned", &display::format_sol(earnings_resp.total_earned));
    display::print_key_value(
        "Unclaimed",
        &display::format_sol(earnings_resp.total_unclaimed),
    );
    println!();

    OutputFormatter::print_token_table(&tokens);

    if !earnings_resp.earnings.is_empty() {
        println!();
        OutputFormatter::print_earnings_summary(&earnings_resp.earnings);
    }

    Ok(())
}

// iteration 54
