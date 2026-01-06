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

