use anyhow::{Context, Result};
use colored::Colorize;

use crate::api::client::ApiClient;
use crate::cli::app::LeaderboardArgs;
use crate::cli::output::OutputFormatter;
use crate::cli::parser::parse_period;
use crate::config::settings::Settings;

pub async fn execute(args: LeaderboardArgs, config: &Settings) -> Result<()> {
    let period = parse_period(&args.period).map_err(|e| anyhow::anyhow!("{}", e))?;

    let api = ApiClient::new(&config.api_base_url);
    let response = api
        .get_leaderboard(&period, &args.sort, args.limit)
        .await
        .context("Failed to fetch leaderboard")?;

    OutputFormatter::print_leaderboard(&response.entries, &period);

    println!(
        "\n  Sorted by: {}  |  Showing {} of {} entries",
        args.sort.cyan(),
        response.entries.len(),
        response.total
    );

    Ok(())
}

// iteration 17
