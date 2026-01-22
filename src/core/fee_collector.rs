use anyhow::{Context, Result};
use log::info;

use crate::api::client::ApiClient;
use crate::api::types::{ClaimRequest, ClaimResponse};
use crate::models::Earning;

/// Manages fee collection and claiming for agent earnings on launched tokens.
pub struct FeeCollector {
    api: ApiClient,
}

impl FeeCollector {
    /// Create a new FeeCollector with the given API client.
    pub fn new(api: ApiClient) -> Self {
        Self { api }
    }

    /// Retrieve all unclaimed earnings for the specified agent.
    pub async fn get_unclaimed(&self, agent_id: &str) -> Result<Vec<Earning>> {
        let response = self
            .api
            .get_earnings(agent_id)
            .await
            .context("Failed to fetch earnings")?;

        let unclaimed: Vec<Earning> = response
            .earnings
            .into_iter()
            .filter(|e| !e.claimed)
            .collect();

        info!("Found {} unclaimed earnings for agent {}", unclaimed.len(), agent_id);
        Ok(unclaimed)
    }

    /// Claim all available earnings for the specified agent.
    pub async fn claim_all(&self, agent_id: &str) -> Result<ClaimResponse> {
        let request = ClaimRequest {
            agent_id: agent_id.to_string(),
            token_id: None,
            claim_all: true,
        };

