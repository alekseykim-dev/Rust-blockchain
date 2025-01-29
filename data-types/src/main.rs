use sha3::{Digest, Keccak256};

const L1_GAS_PER_TX: u32 = 21000; 
const L2_BATCH_GAS: u32 = 70000; 
 
fn main() {
    let l2_address = "0xf7757fF100FC0A1932EC9f5D5CD2A91ff423a2A0";

    // Simulated transactions
    let transactions = vec![
        "Taki -> Vitalik: 1 ETH",
        "Amy -> Brian: 0.5 ETH",
        "Alex -> Kim: 3 ETH",
        "Kate -> Alex: 2.5 ETH",
    ];

    // Count transactions
    let transaction_count = transactions.len();
    println!("L2 Contract Address: {}", l2_address);
    println!("Number of Transactions: {}", transaction_count);

    // Calculate L1 gas cost
    let l1_gas_cost = calculate_l1_gas(transaction_count);
    println!("Total Gas Cost on Layer 1: {} gas", l1_gas_cost);

    // Compare to L2 batch gas cost and calculate savings
    match calculate_gas_savings(l1_gas_cost, L2_BATCH_GAS) {
        Some(savings) => println!("Gas Saved Using Layer 2: {} gas", savings),
        None => println!("Gas Saved Using Layer 2: Not applicable (L2 cost exceeds L1)"),
    }

    // Hash transactions (simulate transaction processing)
    println!("\nProcessing Transactions:");
    for tx in &transactions {
        let tx_hash = hash_transaction(tx);
        println!("  {} -> Hash: {}", tx, tx_hash);
    }

    // Calculate a Merkle Root for the batch
    let batch_root = calculate_merkle_root(&transactions);
    println!("\nBatch Merkle Root: {}", batch_root);
}

fn calculate_l1_gas(tx_count: usize) -> u32 {
    (tx_count as u32) * L1_GAS_PER_TX
}

fn calculate_gas_savings(l1_gas_cost: u32, l2_batch_gas: u32) -> Option<u32> {
    if l1_gas_cost >= l2_batch_gas {
        Some(l1_gas_cost - l2_batch_gas)
    } else {
        None
    }
} 

fn hash_transaction(transaction: &str) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(transaction); 
    hex::encode(hasher.finalize()) 
}


fn calculate_merkle_root(transactions: &Vec<&str>) -> String {
    let mut hasher = Keccak256::new();
    for tx in transactions {
        hasher.update(tx); 
    }
    hex::encode(hasher.finalize())
}
