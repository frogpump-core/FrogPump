use anyhow::{bail, Result};

/// Metadata describing an account referenced by an instruction.
#[derive(Debug, Clone)]
pub struct AccountMeta {
    pub pubkey: [u8; 32],
    pub is_signer: bool,
    pub is_writable: bool,
}

/// A single instruction to be included in a transaction.
#[derive(Debug, Clone)]
pub struct Instruction {
    pub program_id: [u8; 32],
    pub accounts: Vec<AccountMeta>,
    pub data: Vec<u8>,
}

/// Builder for assembling Solana transactions.
///
/// This is a simplified transaction builder for FrogPump's needs.
/// For full Solana transaction support, use the solana-sdk crate directly.
pub struct TransactionBuilder {
    instructions: Vec<Instruction>,
    fee_payer: Option<[u8; 32]>,
    recent_blockhash: Option<String>,
    signatures: Vec<Vec<u8>>,
}

impl TransactionBuilder {
    /// Create a new empty transaction builder.
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            fee_payer: None,
            recent_blockhash: None,
            signatures: Vec::new(),
        }
    }

    /// Add an instruction to the transaction.
    pub fn add_instruction(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);
        self
    }

    /// Set the fee payer's public key for the transaction.
    pub fn set_fee_payer(&mut self, pubkey: [u8; 32]) -> &mut Self {
        self.fee_payer = Some(pubkey);
        self
    }

    /// Set the recent blockhash for the transaction.
    pub fn set_recent_blockhash(&mut self, hash: String) -> &mut Self {
        self.recent_blockhash = Some(hash);
        self
    }

    /// Sign the transaction message with the provided keypair bytes.
    ///
    /// The keypair should be 64 bytes: 32-byte secret key followed by 32-byte public key.
    pub fn sign(&mut self, keypair: &[u8; 64]) -> Result<&mut Self> {
        use ed25519_dalek::{Signer, SigningKey};

        let secret_bytes: [u8; 32] = keypair[..32]
            .try_into()
            .map_err(|_| anyhow::anyhow!("Invalid keypair: could not extract secret key"))?;

        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let message = self.build_message()?;
        let signature = signing_key.sign(&message);
        self.signatures.push(signature.to_bytes().to_vec());
        Ok(self)
    }

    /// Build the serialized transaction bytes.
    ///
    /// Returns a simplified byte representation. In production, this would
    /// produce a fully Solana-compatible binary transaction.
    pub fn build(&self) -> Result<Vec<u8>> {
        if self.instructions.is_empty() {
            bail!("Transaction must have at least one instruction");
        }
        if self.fee_payer.is_none() {
            bail!("Transaction must have a fee payer");
        }
        if self.recent_blockhash.is_none() {
            bail!("Transaction must have a recent blockhash");
        }

        let message = self.build_message()?;
        let mut tx_bytes = Vec::new();

        // Compact array length for signatures
        tx_bytes.push(self.signatures.len() as u8);
        for sig in &self.signatures {
            tx_bytes.extend_from_slice(sig);
        }
        tx_bytes.extend_from_slice(&message);

        Ok(tx_bytes)
    }

    /// Build the transaction message (the signable payload).
    fn build_message(&self) -> Result<Vec<u8>> {
        let mut message = Vec::new();

        // Simplified message header
        let num_required_signatures: u8 = 1;
        let num_readonly_signed: u8 = 0;
        let num_readonly_unsigned: u8 = 0;
        message.push(num_required_signatures);
        message.push(num_readonly_signed);
        message.push(num_readonly_unsigned);

        // Fee payer
        if let Some(ref payer) = self.fee_payer {
            message.extend_from_slice(payer);
        }

        // Blockhash
        if let Some(ref hash) = self.recent_blockhash {
            message.extend_from_slice(hash.as_bytes());
        }

        // Instructions (simplified encoding)
        message.push(self.instructions.len() as u8);
        for ix in &self.instructions {
            message.extend_from_slice(&ix.program_id);
            message.push(ix.accounts.len() as u8);
            for acct in &ix.accounts {
                message.extend_from_slice(&acct.pubkey);
                message.push(u8::from(acct.is_signer));
                message.push(u8::from(acct.is_writable));
            }
            message.push(ix.data.len() as u8);
            message.extend_from_slice(&ix.data);
        }

        Ok(message)
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// iteration 50
