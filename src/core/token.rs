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

    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn image_url(mut self, image_url: Option<String>) -> Self {
        self.image_url = image_url;
        self
    }

    pub fn agent_id(mut self, agent_id: String) -> Self {
        self.agent_id = Some(agent_id);
        self
    }

    pub fn launch_type(mut self, launch_type: LaunchType) -> Self {
        self.launch_type = launch_type;
        self
    }

    /// Validate all fields and build the final LaunchRequest.
    pub fn build(self) -> Result<LaunchRequest> {
        validator::validate_token_name(&self.name)?;
        validator::validate_symbol(&self.symbol)?;

        let agent_id = match self.agent_id {
            Some(id) => id,
            None => bail!("Agent ID is required to build a launch request"),
        };

        validator::validate_agent_id(&agent_id)?;

        let self_funded = matches!(self.launch_type, LaunchType::SelfFunded);

        Ok(LaunchRequest {
            name: self.name,
            symbol: self.symbol,
            description: self.description,
            image_url: self.image_url,
            agent_id,
            self_funded,
        })
    }
}
