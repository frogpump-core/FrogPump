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

impl Agent {
    pub fn has_wallet(&self) -> bool {
        self.wallet_address.is_some()
    }

    pub fn short_wallet(&self) -> Option<String> {
        self.wallet_address.as_ref().map(|addr| {
            if addr.len() >= 8 {
                format!("{}...{}", &addr[..4], &addr[addr.len() - 4..])
            } else {
                addr.clone()
            }
        })
    }
}
