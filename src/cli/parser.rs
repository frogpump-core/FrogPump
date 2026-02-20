use crate::utils::crypto::is_valid_base58;
use crate::utils::error::FrogError;

/// Validate a token symbol: uppercase ASCII alphanumeric, 1-10 characters.
pub fn validate_symbol(symbol: &str) -> Result<String, FrogError> {
    let trimmed = symbol.trim();
    if trimmed.is_empty() || trimmed.len() > 10 {
        return Err(FrogError::Validation(
            "Symbol must be 1-10 characters".into(),
        ));
    }
    let upper = trimmed.to_uppercase();
    if !upper.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(FrogError::Validation(
            "Symbol must contain only alphanumeric ASCII characters".into(),
        ));
    }
    Ok(upper)
}

/// Validate that an agent ID is non-empty and well-formed.
pub fn validate_agent_id(id: &str) -> Result<String, FrogError> {
    let trimmed = id.trim();
    if trimmed.is_empty() {
        return Err(FrogError::Validation("Agent ID must not be empty".into()));
    }
    if trimmed.len() > 64 {
        return Err(FrogError::Validation(
            "Agent ID must be 64 characters or fewer".into(),
        ));
    }
    Ok(trimmed.to_string())
}

/// Validate a Solana address: base58 encoded, 32-44 characters.
pub fn validate_address(address: &str) -> Result<String, FrogError> {
    let trimmed = address.trim();
    if trimmed.len() < 32 || trimmed.len() > 44 {
        return Err(FrogError::Validation(
            "Address must be 32-44 characters (base58)".into(),
        ));
    }
    if !is_valid_base58(trimmed) {
        return Err(FrogError::Validation(
            "Address is not valid base58".into(),
        ));
    }
    Ok(trimmed.to_string())
}

/// Parse a leaderboard period string into a canonical form.
/// Accepts: 24h, 7d, 30d, all (with aliases 1d, 1w, 1m).
pub fn parse_period(period: &str) -> Result<String, FrogError> {
    match period.to_lowercase().as_str() {
        "24h" | "1d" => Ok("24h".to_string()),
        "7d" | "1w" => Ok("7d".to_string()),
        "30d" | "1m" => Ok("30d".to_string()),
        "all" => Ok("all".to_string()),
        _ => Err(FrogError::Validation(format!(
            "Invalid period '{}'. Use 24h, 7d, 30d, or all",
            period
        ))),
    }
}

// iteration 22
