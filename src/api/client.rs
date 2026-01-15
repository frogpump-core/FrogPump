use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use log::debug;
use reqwest::multipart;

use crate::api::endpoints;
use crate::api::response::ApiResponse;
use crate::api::types::{
    ClaimRequest, ClaimResponse, EarningsResponse, LaunchRequest, LaunchResponse,
    LeaderboardResponse, StatsResponse, WalletRequest,
};
use crate::models::Token;

/// HTTP client for interacting with the FrogPump REST API.
pub struct ApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    /// Create a new ApiClient targeting the given base URL.
    pub fn new(base_url: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
        }
    }

    /// Launch a new token. Dispatches to gasless or self-funded endpoint based on the request.
    pub async fn launch(&self, request: LaunchRequest) -> Result<LaunchResponse> {
        let endpoint = if request.self_funded {
            endpoints::LAUNCH_SELF_FUNDED
        } else {
            endpoints::LAUNCH
        };
        let url = endpoints::build_url(&self.base_url, endpoint, &[]);
        debug!("POST {}", url);

