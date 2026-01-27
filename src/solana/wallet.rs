use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use log::debug;

use crate::solana::keypair;

/// Manages local wallet keypair storage and signing operations.
pub struct WalletManager {
    config_dir: PathBuf,
}

impl WalletManager {
    /// Create a new WalletManager using the application config directory.
    pub fn new() -> Self {
        Self {
            config_dir: crate::config_dir(),
        }
    }

    /// Load the keypair bytes from the local wallet file.
    pub fn load_keypair(&self) -> Result<[u8; 64]> {
        let path = self.config_dir.join("wallet.json");
        debug!("Loading keypair from {}", path.display());

        let data = std::fs::read_to_string(&path)
            .context(format!("No wallet found at {}", path.display()))?;

        let bytes: Vec<u8> =
            serde_json::from_str(&data).context("Invalid wallet file format")?;

        if bytes.len() != 64 {
            bail!(
                "Invalid keypair length: expected 64 bytes, got {}",
                bytes.len()
            );
        }

        let mut keypair = [0u8; 64];
        keypair.copy_from_slice(&bytes);
        Ok(keypair)
    }

    /// Save keypair bytes to the local wallet file.
    pub fn save_keypair(&self, bytes: &[u8; 64]) -> Result<()> {
        std::fs::create_dir_all(&self.config_dir)
            .context("Failed to create config directory")?;

        let path = self.config_dir.join("wallet.json");
        let json = serde_json::to_string(&bytes.to_vec())
            .context("Failed to serialize keypair")?;

        std::fs::write(&path, json).context(format!("Failed to write wallet to {}", path.display()))?;
        debug!("Keypair saved to {}", path.display());
        Ok(())
    }

