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

