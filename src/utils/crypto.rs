use crate::utils::error::{FrogError, Result};

const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn is_valid_base58(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    s.chars().all(|c| BASE58_ALPHABET.contains(c))
}

pub fn is_valid_solana_address(addr: &str) -> bool {
    if !is_valid_base58(addr) {
        return false;
    }
    let len = addr.len();
    (32..=44).contains(&len)
}

pub fn encode_base58(bytes: &[u8]) -> String {
    bs58::encode(bytes).into_string()
}

pub fn decode_base58(s: &str) -> Result<Vec<u8>> {
    bs58::decode(s)
        .into_vec()
        .map_err(|e| FrogError::Parse(format!("Invalid base58: {}", e)))
}

pub fn hash_message(msg: &[u8]) -> Vec<u8> {
    use ed25519_dalek::Sha512;
    use ed25519_dalek::Digest;
    let mut hasher = Sha512::new();
    hasher.update(msg);
    hasher.finalize().to_vec()
}

// iteration 25
