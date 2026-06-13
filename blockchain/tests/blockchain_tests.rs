// Integration tests for the blockchain library

// Bring the modules into scope from the crate root.
use blockchain::block::Block;
use blockchain::blockchain::Blockchain;

#[test]
fn genesis_block_has_correct_index() {
    let chain = Blockchain::new();
    assert_eq!(chain.chain[0].index, 0);
}

#[test]
fn genesis_block_previous_hash_is_zeros() {
    let chain = Blockchain::new();
    assert_eq!(chain.chain[0].previous_hash, "0".repeat(64));
}

#[test]
fn genesis_block_hash_is_valid() {
    let chain = Blockchain::new();
    assert!(chain.chain[0].is_valid_hash());
}

#[test]
fn add_block_increments_index() {
    let mut chain = Blockchain::new();
    chain.add_block("tx1".to_string());
    chain.add_block("tx2".to_string());
    assert_eq!(chain.chain[1].index, 1);
    assert_eq!(chain.chain[2].index, 2);
}

#[test]
fn blocks_are_linked_by_hash() {
    let mut chain = Blockchain::new();
    chain.add_block("tx".to_string());
    assert_eq!(chain.chain[1].previous_hash, chain.chain[0].hash);
}

#[test]
fn valid_chain_passes_validation() {
    let mut chain = Blockchain::new();
    chain.add_block("Alice sent 10 to Bob".to_string());
    chain.add_block("Bob sent 5 to Carol".to_string());
    assert!(chain.validate().is_ok());
}

#[test]
fn tampered_data_fails_validation() {
    let mut chain = Blockchain::new();
    chain.add_block("Alice sent 10 to Bob".to_string());
    chain.tamper(1, "Alice sent 9999 to Bob".to_string()).unwrap();
    assert!(chain.validate().is_err());
}

#[test]
fn tampered_hash_propagation_detected() {
    let mut chain = Blockchain::new();
    chain.add_block("tx1".to_string());
    chain.add_block("tx2".to_string());
    // Tamper middle block
    chain.tamper(1, "injected".to_string()).unwrap();
    let err = chain.validate().unwrap_err();
    assert!(err.contains("#1"), "Error should mention block #1");
}

#[test]
fn compute_hash_is_deterministic() {
    let h1 = Block::compute_hash(1, 1_700_000_000, "data", "prevhash");
    let h2 = Block::compute_hash(1, 1_700_000_000, "data", "prevhash");
    assert_eq!(h1, h2);
}

#[test]
fn compute_hash_differs_on_changed_data() {
    let h1 = Block::compute_hash(1, 1_700_000_000, "original", "prevhash");
    let h2 = Block::compute_hash(1, 1_700_000_000, "modified", "prevhash");
    assert_ne!(h1, h2);
}

#[test]
fn tamper_out_of_range_returns_error() {
    let mut chain = Blockchain::new();
    assert!(chain.tamper(99, "x".to_string()).is_err());
}
