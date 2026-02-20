use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LaunchType {
    Gasless,
    SelfFunded,
}

impl fmt::Display for LaunchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LaunchType::Gasless => write!(f, "Gasless"),
            LaunchType::SelfFunded => write!(f, "Self-Funded"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub mint_address: String,
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub agent_id: String,
    pub launch_type: LaunchType,
    pub verified: bool,
    pub created_at: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) [{}] - {}",
            self.name,
            self.symbol,
            self.short_address(),
            self.launch_type
        )
    }
}

impl Token {
    pub fn short_address(&self) -> String {
        if self.mint_address.len() >= 8 {
            format!(
                "{}...{}",
                &self.mint_address[..4],
                &self.mint_address[self.mint_address.len() - 4..]
            )
        } else {
            self.mint_address.clone()
        }
    }

    pub fn pump_fun_url(&self) -> String {
        format!("https://pump.fun/coin/{}", self.mint_address)
    }

    pub fn is_gasless(&self) -> bool {
        self.launch_type == LaunchType::Gasless
    }
}

// iteration 28
