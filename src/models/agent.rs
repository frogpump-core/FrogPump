use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub agent_id: String,
    pub wallet_address: Option<String>,
    pub created_at: String,
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Agent {} [wallet: {}]",
            self.agent_id,
            self.short_wallet().unwrap_or_else(|| "none".to_string())
        )
    }
}

