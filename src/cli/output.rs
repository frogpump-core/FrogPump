use colored::Colorize;
use tabled::{Table, Tabled};

use crate::api::types::{LeaderboardEntry, StatsResponse};
use crate::models::earning::Earning;
use crate::models::token::Token;
use crate::utils::display;

#[derive(Tabled)]
struct TokenRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Symbol")]
    symbol: String,
    #[tabled(rename = "Mint Address")]
    mint_address: String,
    #[tabled(rename = "Verified")]
    verified: String,
}

#[derive(Tabled)]
struct EarningRow {
    #[tabled(rename = "Token")]
    token_id: String,
    #[tabled(rename = "Amount (SOL)")]
    amount: String,
    #[tabled(rename = "Claimed")]
    claimed: String,
}

#[derive(Tabled)]
struct LeaderboardRow {
    #[tabled(rename = "Rank")]
    rank: String,
    #[tabled(rename = "Token")]
    token: String,
    #[tabled(rename = "Volume 24h")]
    volume: String,
    #[tabled(rename = "Market Cap")]
    market_cap: String,
    #[tabled(rename = "Earnings")]
    earnings: String,
}

