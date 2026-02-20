use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// Generic API response wrapper used across all endpoints.
///
/// On success, `success` is true and `data` contains the response payload.
/// On failure, `success` is false and `error` contains a description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    /// Convert the API response into a Result, extracting the data on success
    /// or returning the error message on failure.
    pub fn into_result(self) -> Result<T> {
        if self.success {
            match self.data {
                Some(data) => Ok(data),
                None => bail!("API returned success but no data payload"),
            }
        } else {
            let msg = self.error.unwrap_or_else(|| "Unknown API error".to_string());
            bail!("API error: {}", msg)
        }
    }
}

/// Paginated API response wrapper for list endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub per_page: u32,
    pub total: u32,
}

impl<T> PaginatedResponse<T> {
    /// Check if there are more pages available.
    pub fn has_more(&self) -> bool {
        (self.page * self.per_page) < self.total
    }
}

// iteration 8
