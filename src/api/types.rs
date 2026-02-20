use serde::{Deserialize, Serialize};

use crate::models::Earning;

/// Request payload for launching a new token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchRequest {
    pub name: String,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub agent_id: String,
    pub self_funded: bool,
}

/// Response returned after a successful token launch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchResponse {
    pub mint_address: String,
    pub tx_signature: String,
    pub pump_fun_url: String,
}

/// Response containing earnings data for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsResponse {
    pub earnings: Vec<Earning>,
    pub total_earned: f64,
    pub total_unclaimed: f64,
}

/// Request payload for claiming earnings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimRequest {
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    pub claim_all: bool,
}

/// Response returned after a successful claim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponse {
    pub amount: f64,
    pub tx_signature: String,
}

/// Request payload for associating a wallet with an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletRequest {
    pub agent_id: String,
    pub wallet_address: String,
    pub signature: String,
    pub message: String,
}

/// A single entry on the leaderboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: u32,
    pub token_name: String,
    pub symbol: String,
    pub volume_24h: f64,
    pub market_cap: f64,
    pub creator_earnings: f64,
    pub mint_address: String,
}

/// Response containing leaderboard data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    pub entries: Vec<LeaderboardEntry>,
    pub total: u32,
}

/// Response containing platform-wide statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub total_tokens: u64,
    pub total_earnings_sol: f64,
    pub active_agents: u64,
    pub volume_24h: f64,
}

// iteration 86
