use clap::Parser;
use frogpump::cli::app::{Cli, Commands};

#[tokio::main]
async fn main() {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Launch { .. } => {
            println!("Launch command - not yet implemented");
        }
        _ => {
            eprintln!("Command not yet implemented");
        }
    }
}
