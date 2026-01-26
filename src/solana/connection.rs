use anyhow::{bail, Context, Result};
use log::debug;
use serde_json::{json, Value};

/// Manages a connection to a Solana RPC node.
pub struct SolanaConnection {
    rpc_url: String,
    ws_url: Option<String>,
    client: reqwest::Client,
}

impl SolanaConnection {
    /// Create a new connection to the specified RPC URL.
    pub fn new(rpc_url: &str) -> Self {
        let ws_url = rpc_url
            .replace("https://", "wss://")
            .replace("http://", "ws://");

        Self {
            rpc_url: rpc_url.to_string(),
            ws_url: Some(ws_url),
            client: reqwest::Client::new(),
        }
    }

    /// Send a JSON-RPC request to the Solana node.
    async fn rpc_request(&self, method: &str, params: Value) -> Result<Value> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        });

        debug!("RPC {} -> {}", method, self.rpc_url);

        let resp = self
            .client
            .post(&self.rpc_url)
            .json(&body)
            .send()
            .await
            .context(format!("RPC request failed: {}", method))?;

        let json: Value = resp.json().await.context("Failed to parse RPC response")?;

        if let Some(error) = json.get("error") {
            bail!("RPC error ({}): {}", method, error);
        }

        Ok(json)
    }

    /// Check if the RPC node is healthy and reachable.
    pub async fn health_check(&self) -> Result<bool> {
        let result = self.rpc_request("getHealth", json!([])).await;
        Ok(result.is_ok())
    }

