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

