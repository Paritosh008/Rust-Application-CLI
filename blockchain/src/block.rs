// block.rs — defines the Block struct and its core logic

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Represents a single block in the blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Position of this block in the chain (0 = genesis)
    pub index: u64,
    /// Unix timestamp (seconds since epoch) when the block was created
    pub timestamp: i64,
    /// Arbitrary transaction/message data stored in this block
    pub data: String,
    /// SHA-256 hash of the previous block (all zeros for genesis)
    pub previous_hash: String,
    /// SHA-256 hash of this block's contents
    pub hash: String,
}

impl Block {
    /// Creates a new block and immediately computes its hash.
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let hash = Self::compute_hash(index, timestamp, &data, &previous_hash);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    /// Creates the genesis block (index 0) with a zeroed previous hash.
    pub fn genesis() -> Self {
        Block::new(0, String::from("Genesis Block"), String::from("0".repeat(64)))
    }

    /// Computes the SHA-256 hash for the given block fields.
    /// The input is the concatenation: index + timestamp + data + previous_hash.
    pub fn compute_hash(index: u64, timestamp: i64, data: &str, previous_hash: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Recomputes the hash from stored fields and compares against stored hash.
    /// Returns true if the block is internally consistent.
    pub fn is_valid_hash(&self) -> bool {
        let recalculated = Self::compute_hash(
            self.index,
            self.timestamp,
            &self.data,
            &self.previous_hash,
        );
        recalculated == self.hash
    }

    /// Pretty-prints the block contents.
    pub fn display(&self) {
        println!("┌─────────────────────────────────────────────────────────────────┐");
        println!("│ Block #{:<61}│", self.index);
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ Timestamp    : {:<51}│", self.timestamp);
        println!("│ Data         : {:<51}│", truncate(&self.data, 51));
        println!("│ Prev Hash    : {:<51}│", truncate(&self.previous_hash, 51));
        println!("│ Hash         : {:<51}│", truncate(&self.hash, 51));
        println!("└─────────────────────────────────────────────────────────────────┘");
    }
}

/// Truncates a string to `max_len` characters, adding "…" if needed.
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}…", &s[..max_len - 1])
    }
}
