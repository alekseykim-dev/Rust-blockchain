use sha2::{Sha256, Digest};
use chrono::Utc;

// Rollup Block structure
#[derive(Debug)]
struct BaseRollupBlock {
    index: u32,
    timestamp: String,
    data: String,
    previous_hash: String,
    state_root: String,
    hash: String,
}
impl BaseRollupBlock {
    // print block details
    fn display(&self) {
        println!("--------------------------");
        println!("Base Rollup Block #{}", self.index);
        println!("Timestamp: {}", self.timestamp);
        println!("Transaction Data: {}", self.data);
        println!("Previous Hash: {}", self.previous_hash);
        println!("State Root: {}", self.state_root);
        println!("Block Hash: {}", self.hash);
        println!("--------------------------\n");
    }
}
// calculate SHA-256 hash
fn calculate_hash(index: u32, timestamp: &str, data: &str, previous_hash: &str, state_root: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}{}{}{}", index, timestamp, data, previous_hash, state_root));
    format!("{:x}", hasher.finalize())
}
//  simulate state root calculation
fn calculate_state_root(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

// new Base Rollup Block
fn create_base_rollup_block(index: u32, data: &str, previous_hash: &str) -> BaseRollupBlock {
    let timestamp = Utc::now().to_string();
    let state_root = calculate_state_root(data);
    let hash = calculate_hash(index, &timestamp, data, previous_hash, &state_root);

    BaseRollupBlock {
        index,
        timestamp,
        data: data.to_string(),
        previous_hash: previous_hash.to_string(),
        state_root,
        hash,
    }
}
fn main() {
    // Genesis Base Rollup block
    let genesis_block = create_base_rollup_block(0, "Genesis Base Rollup Block", "0");
    genesis_block.display();

    // transactions
    let block_1 = create_base_rollup_block(1, "Base Transaction: Alex sends 50 tokens to Taki", &genesis_block.hash);
    block_1.display();

    let block_2 = create_base_rollup_block(2, "Base Transaction: Kathie bridges 20 tokens to Ethereum ", &block_1.hash);
    block_2.display();

    // fraud-proof verification logic (for optimistic rollups)
    println!("Fraud-proof mechanism placeholder: Validate transactions against state roots.");
}
