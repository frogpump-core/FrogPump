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

