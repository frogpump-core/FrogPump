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

    Ok(())
}

/// Validate a Solana address: base58-encoded string of 32-44 characters.
pub fn validate_solana_address(address: &str) -> Result<()> {
    if address.len() < 32 || address.len() > 44 {
        bail!(
            "Solana address must be 32-44 characters, got {}",
            address.len()
        );
    }

    let base58_chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    if !address.chars().all(|c| base58_chars.contains(c)) {
        bail!("Solana address contains invalid base58 characters: '{}'", address);
    }

    Ok(())
}

/// Validate an agent identifier: non-empty, 1-64 characters, alphanumeric with hyphens/underscores.
pub fn validate_agent_id(id: &str) -> Result<()> {
    if id.is_empty() || id.len() > 64 {
        bail!("Agent ID must be between 1 and 64 characters, got {}", id.len());
    }

    if !id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        bail!(
            "Agent ID must contain only alphanumeric characters, hyphens, and underscores: '{}'",
            id
        );
    }

    Ok(())
}

// iteration 4
