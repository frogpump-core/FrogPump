use anyhow::{bail, Result};

use crate::api::types::LaunchRequest;
use crate::core::validator;
use crate::models::LaunchType;

/// Builder for constructing validated token launch requests.
pub struct TokenBuilder {
    name: String,
    symbol: String,
    description: Option<String>,
    image_url: Option<String>,
    agent_id: Option<String>,
    launch_type: LaunchType,
}

impl TokenBuilder {
    /// Create a new TokenBuilder with required name and symbol.
    pub fn new(name: String, symbol: String) -> Self {
        Self {
            name,
            symbol,
            description: None,
            image_url: None,
            agent_id: None,
            launch_type: LaunchType::Gasless,
        }
    }

