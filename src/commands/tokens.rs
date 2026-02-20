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

    OutputFormatter::print_token_table(&tokens);

    if verbose {
        for token in &tokens {
            println!();
            display::print_divider();
            println!("  {} ({})", token.name, token.symbol);
            display::print_key_value("Mint", &token.mint_address);
            display::print_key_value("Launch type", &token.launch_type.to_string());
            if let Some(ref desc) = token.description {
                display::print_key_value("Description", desc);
            }
            if let Some(ref img) = token.image_url {
                display::print_key_value("Image", img);
            }
            display::print_key_value("Created", &display::format_timestamp(&token.created_at));
        }
    }

    println!("\n  Total: {} token(s)", tokens.len());

    Ok(())
}

// iteration 99
