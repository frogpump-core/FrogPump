use anyhow::{Context, Result};

use crate::api::client::ApiClient;
use crate::cli::output::OutputFormatter;
use crate::config::settings::Settings;
use crate::utils::display;

pub async fn execute(config: &Settings) -> Result<()> {
    let api = ApiClient::new(&config.api_base_url);

    let stats = api
        .get_stats()
        .await
        .context("Failed to fetch platform stats")?;

    println!();
    OutputFormatter::print_stats(&stats);

    display::print_key_value(
        "Network",
        &format!("{:?}", config.network),
    );
    display::print_key_value("RPC", &config.rpc_url);
    println!();

    Ok(())
}

// iteration 100
