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

