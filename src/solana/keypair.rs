use anyhow::{bail, Context, Result};
use ed25519_dalek::SigningKey;

/// Generate a new random ed25519 keypair.
///
/// Returns a tuple of (secret_key, public_key) as 32-byte arrays.
pub fn generate_keypair() -> ([u8; 32], [u8; 32]) {
    let mut csprng = rand::rngs::OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let public_key = signing_key.verifying_key();

    (signing_key.to_bytes(), public_key.to_bytes())
}

/// Convert raw public key bytes to a base58-encoded Solana address string.
pub fn pubkey_from_bytes(bytes: &[u8]) -> Result<String> {
    if bytes.len() != 32 {
        bail!("Public key must be exactly 32 bytes, got {}", bytes.len());
    }
    Ok(bs58::encode(bytes).into_string())
}

/// Decode a base58 Solana address string into raw bytes.
pub fn bytes_from_pubkey(address: &str) -> Result<Vec<u8>> {
    let bytes = bs58::decode(address)
        .into_vec()
        .context(format!("Invalid base58 address: {}", address))?;

    if bytes.len() != 32 {
        bail!(
            "Decoded address is {} bytes, expected 32",
            bytes.len()
        );
    }

