use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Earning {
    pub id: String,
    pub token_id: String,
    pub agent_id: String,
    pub amount: f64,
    pub claimed: bool,
    pub claimed_at: Option<String>,
    pub created_at: String,
}

impl fmt::Display for Earning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.claimed {
            "claimed"
        } else {
            "unclaimed"
        };
        write!(
            f,
            "Earning {} [{}] - {} ({})",
            self.id,
            self.token_id,
            self.format_amount(),
            status
        )
    }
}

impl Earning {
    pub fn is_claimable(&self) -> bool {
        !self.claimed && self.amount > 0.0
    }

    pub fn format_amount(&self) -> String {
        format!("{:.4} SOL", self.amount)
    }
}
