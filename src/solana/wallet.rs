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

    /// Derive the base58-encoded public key from the stored keypair.
    pub fn get_public_key(&self) -> Result<String> {
        let kp = self.load_keypair()?;
        keypair::pubkey_from_bytes(&kp[32..])
    }

    /// Sign an arbitrary message with the stored keypair.
    pub fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>> {
        let kp = self.load_keypair()?;
        let secret: [u8; 32] = kp[..32].try_into().context("Invalid secret key bytes")?;
        let signing_key = SigningKey::from_bytes(&secret);
        let signature = signing_key.sign(message);
        Ok(signature.to_bytes().to_vec())
    }

    /// Verify a signature against a public key and message.
    pub fn verify_signature(
        pubkey_bytes: &[u8],
        message: &[u8],
        signature_bytes: &[u8],
    ) -> Result<bool> {
        let pubkey: [u8; 32] = pubkey_bytes
            .try_into()
            .context("Public key must be 32 bytes")?;
        let sig_bytes: [u8; 64] = signature_bytes
            .try_into()
            .context("Signature must be 64 bytes")?;

        let verifying_key = VerifyingKey::from_bytes(&pubkey)
            .context("Invalid public key")?;
        let signature = ed25519_dalek::Signature::from_bytes(&sig_bytes);

        Ok(verifying_key.verify(message, &signature).is_ok())
    }
}

// iteration 11
