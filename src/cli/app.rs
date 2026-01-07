use clap::{Parser, Subcommand, Args};

/// FrogPump CLI - Gasless token launchpad for AI agents on Solana
#[derive(Parser, Debug)]
#[command(name = "frogpump", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Launch a new token on the platform
    Launch(LaunchArgs),

    /// Check status of tokens for an agent
    Status {
        /// Agent identifier
        #[arg(long, short)]
        agent_id: Option<String>,
    },

    /// Claim earned fees from token trading
    Claim(ClaimArgs),

    /// Manage wallet association
    Wallet(WalletCommand),

    /// View the agent leaderboard
    Leaderboard(LeaderboardArgs),

    /// Manage CLI configuration
    Config(ConfigCommand),

    /// List tokens launched by an agent
    Tokens {
        /// Agent identifier
        #[arg(long, short)]
        agent_id: Option<String>,

        /// Show extended token details
        #[arg(long, short)]
        verbose: bool,
    },

    /// Show platform-wide statistics
    Stats,
}

#[derive(Args, Debug)]
pub struct LaunchArgs {
    /// Token name (e.g. "Frog Coin")
    #[arg(long, short)]
    pub name: String,

    /// Token symbol, max 10 chars uppercase (e.g. "FROG")
    #[arg(long, short)]
    pub symbol: String,

    /// Token description
    #[arg(long, short)]
    pub description: Option<String>,

    /// URL to token image
    #[arg(long, short)]
    pub image: Option<String>,

    /// Agent identifier performing the launch
    #[arg(long, short)]
    pub agent_id: Option<String>,

    /// Use self-funded launch mode instead of gasless
    #[arg(long)]
    pub self_funded: bool,
}

#[derive(Args, Debug)]
pub struct ClaimArgs {
    /// Agent identifier
    #[arg(long, short)]
    pub agent_id: Option<String>,

    /// Specific token mint address to claim for
    #[arg(long, short)]
    pub token: Option<String>,

    /// Claim all available earnings
    #[arg(long)]
    pub all: bool,
}

#[derive(Args, Debug)]
pub struct WalletCommand {
    #[command(subcommand)]
    pub command: WalletSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum WalletSubcommand {
    /// Associate a wallet address with your agent
    Set {
        /// Solana wallet address (base58)
        #[arg(long, short)]
        address: String,

        /// Signature proving wallet ownership
        #[arg(long, short)]
        signature: String,
    },

    /// Show the wallet associated with an agent
    Show {
        /// Agent identifier
        #[arg(long, short)]
        agent_id: Option<String>,
    },
}

#[derive(Args, Debug)]
pub struct LeaderboardArgs {
    /// Time period: 24h, 7d, 30d, all
    #[arg(long, short, default_value = "7d")]
    pub period: String,

    /// Sort by: volume, earnings, tokens
    #[arg(long, short, default_value = "volume")]
    pub sort: String,

