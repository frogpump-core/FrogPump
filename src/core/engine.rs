use anyhow::{Context, Result};
use log::info;

use crate::api::client::ApiClient;
use crate::api::types::{LaunchRequest, LaunchResponse};
use crate::config::settings::Settings;
use crate::core::token::TokenBuilder;
use crate::core::validator;
use crate::models::LaunchType;

/// Core engine that orchestrates token launches through the FrogPump platform.
pub struct LaunchEngine {
    api: ApiClient,
    config: Settings,
}

impl LaunchEngine {
    /// Create a new LaunchEngine with the given API client and settings.
    pub fn new(api: ApiClient, config: Settings) -> Self {
        Self { api, config }
    }

    /// Execute a token launch with the provided parameters.
    ///
    /// Validates all inputs, builds the launch request, and dispatches
    /// to either gasless or self-funded execution path.
    pub async fn execute_launch(
        &self,
        name: String,
        symbol: String,
        description: Option<String>,
        image_url: Option<String>,
        agent_id: Option<String>,
        self_funded: bool,
    ) -> Result<LaunchResponse> {
        let agent_id = agent_id
            .or_else(|| self.config.agent_id.clone())
            .context("Agent ID is required. Set it via --agent-id or in config.")?;

        validator::validate_token_name(&name)?;
        validator::validate_symbol(&symbol)?;
        validator::validate_agent_id(&agent_id)?;

        let launch_type = if self_funded {
            LaunchType::SelfFunded
        } else {
            LaunchType::Gasless
        };

        let request = TokenBuilder::new(name, symbol)
            .description(description)
            .image_url(image_url)
            .agent_id(agent_id.clone())
            .launch_type(launch_type)
            .build()?;

        info!("Launching token {} ({}) for agent {}", request.name, request.symbol, agent_id);

        if self_funded {
            self.execute_self_funded(request).await
        } else {
            self.execute_gasless(request).await
        }
    }

    /// Execute a gasless launch through the platform's sponsored transaction flow.
    async fn execute_gasless(&self, request: LaunchRequest) -> Result<LaunchResponse> {
        info!("Using gasless launch mode");
        self.api
            .launch(request)
            .await
            .context("Gasless launch failed")
    }

    /// Execute a self-funded launch where the agent pays transaction fees.
    async fn execute_self_funded(&self, mut request: LaunchRequest) -> Result<LaunchResponse> {
        info!("Using self-funded launch mode");
        request.self_funded = true;
        self.api
            .launch(request)
            .await
            .context("Self-funded launch failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let api = ApiClient::new("https://api.frogpump.fun/v1");
        let config = Settings::default();
        let engine = LaunchEngine::new(api, config);
        assert!(engine.config.agent_id.is_none());
    }
}

// iteration 1
