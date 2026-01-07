use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "frogpump")]
#[command(about = "Gasless token launchpad CLI for AI agents on Solana")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Launch a new token on pump.fun
    Launch {
        #[arg(long, short)]
        name: String,
        #[arg(long, short)]
        symbol: String,
    },
}
