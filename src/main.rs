use clap::Parser;
use env_logger::Env;
use log::error;

use frog_pump::cli::app::{Cli, Commands, ConfigSubcommand, WalletSubcommand};
use frog_pump::commands;
use frog_pump::config::settings::Settings;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();
    let config = Settings::load().unwrap_or_default();

    let result = match cli.command {
        Commands::Launch(args) => commands::launch::execute(args, &config).await,
        Commands::Status { agent_id } => commands::status::execute(agent_id, &config).await,
        Commands::Claim(args) => commands::claim::execute(args, &config).await,
        Commands::Wallet(sub) => match sub.command {
            WalletSubcommand::Set { address, signature } => {
                commands::wallet::execute_set(address, signature, &config).await
            }
            WalletSubcommand::Show { agent_id } => {
                commands::wallet::execute_show(agent_id, &config).await
            }
        },
        Commands::Leaderboard(args) => commands::leaderboard::execute(args, &config).await,
        Commands::Config(sub) => match sub.command {
            ConfigSubcommand::Set { key, value } => commands::config_cmd::execute_set(&key, &value),
            ConfigSubcommand::Get { key } => commands::config_cmd::execute_get(&key),
            ConfigSubcommand::Show => commands::config_cmd::execute_show(),
        },
        Commands::Tokens { agent_id, verbose } => {
            commands::tokens::execute(agent_id, verbose, &config).await
        }
        Commands::Stats => commands::stats::execute(&config).await,
    };

    if let Err(e) = result {
        error!("{}", e);
        std::process::exit(1);
    }
}
