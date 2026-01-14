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

