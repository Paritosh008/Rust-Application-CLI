// blockchain.rs — manages the chain of blocks

use crate::block::Block;

/// Holds the ordered list of blocks that form the blockchain.
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    /// Initialises a new blockchain with the genesis block already added.
    pub fn new() -> Self {
        let genesis = Block::genesis();
        println!("Genesis block created → hash: {}", &genesis.hash[..16]);
        Blockchain {
            chain: vec![genesis],
        }
    }

    /// Returns a reference to the most recently added block.
    pub fn latest_block(&self) -> &Block {
        // The chain is never empty (genesis is always present), so this is safe.
        self.chain.last().expect("Chain must have at least one block")
    }

    /// Adds a new block whose previous_hash is taken from the current chain tip.
    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.latest_block().hash.clone();
        let index = self.chain.len() as u64;
        let block = Block::new(index, data, previous_hash);
        println!("Block #{} added → hash: {}", block.index, &block.hash[..16]);
        self.chain.push(block);
    }

    /// Prints every block in the chain to stdout.
    pub fn display(&self) {
        println!("\n═══════════════════ BLOCKCHAIN ══════════════════════");
        for block in &self.chain {
            block.display();
        }
        println!("═════════════════════════════════════════════════════\n");
    }

    /// Validates the entire chain.
    ///
    /// For each block it checks:
    ///   1. The stored hash matches a freshly computed hash (tamper detection).
    ///   2. The stored previous_hash matches the actual hash of the preceding block.
    ///
    /// Returns `Ok(())` when valid, or `Err(String)` describing the first violation.
    pub fn validate(&self) -> Result<(), String> {
        for i in 0..self.chain.len() {
            let block = &self.chain[i];

            // Check 1: internal hash consistency
            if !block.is_valid_hash() {
                return Err(format!(
                    "Block #{} has an invalid hash. Data may have been tampered with.",
                    block.index
                ));
            }

            // Check 2: previous_hash linkage (skip genesis which has no predecessor)
            if i > 0 {
                let previous_block = &self.chain[i - 1];
                if block.previous_hash != previous_block.hash {
                    return Err(format!(
                        "Block #{} previous_hash does not match hash of block #{}.",
                        block.index,
                        previous_block.index
                    ));
                }
            }
        }
        Ok(())
    }

    /// Tampers with block at `index` by overwriting its data field.
    /// The stored hash is NOT updated, so validation will fail afterwards.
    ///
    /// Returns an error if the index is out of range.
    pub fn tamper(&mut self, index: usize, new_data: String) -> Result<(), String> {
        if index >= self.chain.len() {
            return Err(format!(
                "Block index {} is out of range (chain length: {}).",
                index,
                self.chain.len()
            ));
        }
        println!(
            "\n⚠️  TAMPERING: Changing data of block #{} without updating its hash…",
            index
        );
        self.chain[index].data = new_data;
        Ok(())
    }
}
