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

