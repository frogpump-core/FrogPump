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

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn print_token_table(tokens: &[Token]) {
        if tokens.is_empty() {
            println!("{}", "No tokens found.".yellow());
            return;
        }
        let rows: Vec<TokenRow> = tokens
            .iter()
            .map(|t| TokenRow {
                name: t.name.clone(),
                symbol: t.symbol.clone(),
                mint_address: display::short_address(&t.mint_address),
                verified: if t.verified {
                    "Yes".green().to_string()
                } else {
                    "No".red().to_string()
                },
            })
            .collect();
        println!("{}", Table::new(rows));
    }

    pub fn print_earnings_summary(earnings: &[Earning]) {
        if earnings.is_empty() {
            println!("{}", "No earnings found.".yellow());
            return;
        }
        let total: f64 = earnings.iter().map(|e| e.amount).sum();
        let unclaimed: f64 = earnings.iter().filter(|e| !e.claimed).map(|e| e.amount).sum();
        let rows: Vec<EarningRow> = earnings
            .iter()
            .map(|e| EarningRow {
                token_id: display::short_address(&e.token_id),
                amount: display::format_sol(e.amount),
                claimed: if e.claimed {
                    "Yes".green().to_string()
                } else {
                    "No".yellow().to_string()
                },
            })
            .collect();
        println!("{}", Table::new(rows));
        println!(
            "\n  Total: {}  |  Unclaimed: {}",
            display::format_sol(total),
            display::format_sol(unclaimed)
        );
    }

