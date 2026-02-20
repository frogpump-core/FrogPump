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

    let total = FeeCollector::total_unclaimed(&unclaimed);

    // Determine what to claim
    if args.token.is_none() && !args.all {
        anyhow::bail!("Specify --token <mint> or --all to claim earnings.");
    }

    if let Some(ref token) = args.token {
        let matching: Vec<_> = unclaimed.iter().filter(|e| e.token_id == *token).collect();
        if matching.is_empty() {
            anyhow::bail!("No unclaimed earnings found for token {}.", token);
        }
        let token_total: f64 = matching.iter().map(|e| e.amount).sum();
        println!(
            "  About to claim {} for token {}",
            display::format_sol(token_total),
            display::short_address(token)
        );
    } else {
        println!(
            "  About to claim {} across {} earning(s) for agent {}",
            display::format_sol(total),
            unclaimed.len(),
            agent_id
        );
    }

    print!("  Proceed? [y/N] ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if !input.trim().eq_ignore_ascii_case("y") {
        println!("  Claim cancelled.");
        return Ok(());
    }

    let response = if let Some(ref token) = args.token {
        collector.claim_token(&agent_id, token).await?
    } else {
        collector.claim_all(&agent_id).await?
    };

    OutputFormatter::print_success(&format!(
        "Claimed {} for agent {}",
        display::format_sol(response.amount),
        agent_id,
    ));
    display::print_key_value("Transaction", &display::short_address(&response.tx_signature));

    Ok(())
}

// iteration 55
