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

        let resp = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send launch request")?;

        let api_resp: ApiResponse<LaunchResponse> = resp
            .json()
            .await
            .context("Failed to parse launch response")?;

        api_resp.into_result()
    }

    /// Retrieve all tokens launched by the specified agent.
    pub async fn get_tokens(&self, agent_id: &str) -> Result<Vec<Token>> {
        let url = endpoints::build_url(
            &self.base_url,
            endpoints::TOKENS,
            &[("agent_id", agent_id)],
        );
        debug!("GET {}", url);

        let resp = self.client.get(&url).send().await.context("Failed to fetch tokens")?;
        let api_resp: ApiResponse<Vec<Token>> = resp.json().await.context("Failed to parse tokens response")?;
        api_resp.into_result()
    }

    /// Retrieve earnings data for the specified agent.
    pub async fn get_earnings(&self, agent_id: &str) -> Result<EarningsResponse> {
        let url = endpoints::build_url(
            &self.base_url,
            endpoints::EARNINGS,
            &[("agent_id", agent_id)],
        );
        debug!("GET {}", url);

        let resp = self.client.get(&url).send().await.context("Failed to fetch earnings")?;
        let api_resp: ApiResponse<EarningsResponse> = resp.json().await.context("Failed to parse earnings response")?;
        api_resp.into_result()
    }

    /// Submit a claim request for agent earnings.
    pub async fn claim(&self, request: ClaimRequest) -> Result<ClaimResponse> {
        let url = endpoints::build_url(&self.base_url, endpoints::CLAIM, &[]);
        debug!("POST {}", url);

        let resp = self.client.post(&url).json(&request).send().await.context("Failed to send claim request")?;
        let api_resp: ApiResponse<ClaimResponse> = resp.json().await.context("Failed to parse claim response")?;
        api_resp.into_result()
    }

    /// Associate a wallet address with an agent.
    pub async fn set_wallet(&self, request: WalletRequest) -> Result<()> {
        let url = endpoints::build_url(&self.base_url, endpoints::WALLET, &[]);
        debug!("POST {}", url);

        let resp = self.client.post(&url).json(&request).send().await.context("Failed to send wallet request")?;
        let api_resp: ApiResponse<serde_json::Value> = resp.json().await.context("Failed to parse wallet response")?;
        api_resp.into_result()?;
        Ok(())
    }

    /// Fetch the leaderboard with the given filters.
    pub async fn get_leaderboard(
        &self,
        period: &str,
        sort: &str,
        limit: usize,
    ) -> Result<LeaderboardResponse> {
        let limit_str = limit.to_string();
        let url = endpoints::build_url(
            &self.base_url,
            endpoints::LEADERBOARD,
            &[("period", period), ("sort", sort), ("limit", &limit_str)],
        );
        debug!("GET {}", url);

        let resp = self.client.get(&url).send().await.context("Failed to fetch leaderboard")?;
        let api_resp: ApiResponse<LeaderboardResponse> = resp.json().await.context("Failed to parse leaderboard response")?;
        api_resp.into_result()
    }

    /// Fetch platform-wide statistics.
    pub async fn get_stats(&self) -> Result<StatsResponse> {
        let url = endpoints::build_url(&self.base_url, endpoints::STATS, &[]);
        debug!("GET {}", url);

        let resp = self.client.get(&url).send().await.context("Failed to fetch stats")?;
        let api_resp: ApiResponse<StatsResponse> = resp.json().await.context("Failed to parse stats response")?;
        api_resp.into_result()
    }

    /// Upload a token image and return the hosted URL.
    pub async fn upload_image(&self, path: &Path) -> Result<String> {
        let url = endpoints::build_url(&self.base_url, endpoints::UPLOAD, &[]);
        debug!("POST {} (multipart upload)", url);

        let file_bytes = tokio::fs::read(path)
            .await
            .context(format!("Failed to read image file: {}", path.display()))?;

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image.png")
            .to_string();

        let part = multipart::Part::bytes(file_bytes).file_name(file_name);
        let form = multipart::Form::new().part("file", part);

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .context("Failed to upload image")?;

        let api_resp: ApiResponse<String> = resp.json().await.context("Failed to parse upload response")?;
        api_resp.into_result()
    }
}

// iteration 5
