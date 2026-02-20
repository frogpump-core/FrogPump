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

    /// Get the SOL balance for the given address in SOL (not lamports).
    pub async fn get_balance(&self, address: &str) -> Result<f64> {
        let resp = self.rpc_request("getBalance", json!([address])).await?;
        let lamports = resp["result"]["value"]
            .as_u64()
            .context("Invalid balance response")?;
        Ok(lamports as f64 / 1_000_000_000.0)
    }

    /// Get a recent blockhash for transaction signing.
    pub async fn get_recent_blockhash(&self) -> Result<String> {
        let resp = self
            .rpc_request("getLatestBlockhash", json!([{"commitment": "finalized"}]))
            .await?;
        let blockhash = resp["result"]["value"]["blockhash"]
            .as_str()
            .context("Invalid blockhash response")?;
        Ok(blockhash.to_string())
    }

    /// Confirm whether a transaction has been finalized on-chain.
    pub async fn confirm_transaction(&self, signature: &str) -> Result<bool> {
        let resp = self
            .rpc_request(
                "getSignatureStatuses",
                json!([[signature], {"searchTransactionHistory": true}]),
            )
            .await?;

        let statuses = resp["result"]["value"]
            .as_array()
            .context("Invalid signature status response")?;

        if let Some(status) = statuses.first() {
            if status.is_null() {
                return Ok(false);
            }
            let err = status.get("err");
            Ok(err.is_none() || err == Some(&Value::Null))
        } else {
            Ok(false)
        }
    }

    /// Get the WebSocket URL derived from the RPC URL.
    pub fn ws_url(&self) -> Option<&str> {
        self.ws_url.as_deref()
    }
}

// iteration 89
// docs: add module-level documentation
