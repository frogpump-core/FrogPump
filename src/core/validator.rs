use anyhow::{bail, Result};

/// Validate a token name: 1-32 characters, alphanumeric and spaces only.
pub fn validate_token_name(name: &str) -> Result<()> {
    if name.is_empty() || name.len() > 32 {
        bail!("Token name must be between 1 and 32 characters, got {}", name.len());
    }

    if !name.chars().all(|c| c.is_alphanumeric() || c == ' ') {
        bail!(
            "Token name must contain only alphanumeric characters and spaces: '{}'",
            name
        );
    }

    Ok(())
}

/// Validate a token symbol: 1-10 characters, uppercase alphabetic only.
pub fn validate_symbol(symbol: &str) -> Result<()> {
    if symbol.is_empty() || symbol.len() > 10 {
        bail!("Token symbol must be between 1 and 10 characters, got {}", symbol.len());
    }

    if !symbol.chars().all(|c| c.is_ascii_uppercase()) {
        bail!(
            "Token symbol must contain only uppercase letters: '{}'",
            symbol
        );
    }

