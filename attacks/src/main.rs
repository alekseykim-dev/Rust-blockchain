use ethers::{
    middleware::Middleware, // Import Middleware trait
    providers::{Http, Provider},
    types::{Transaction, H256, BlockNumber},
};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use eyre::Result;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); 

    let rpc_url = env::var("BASE_RPC_URL").expect("BASE_RPC_URL not set in .env file");
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);

    // shared memory for storing recent transactions
    let tx_history = Arc::new(Mutex::new(HashMap::<H256, (u64, u128)>::new()));

    println!("🚀 Monitoring latest blocks on Base...");

    loop {
     // latest block
        if let Ok(block) = provider.get_block_with_txs(BlockNumber::Latest).await {
            if let Some(block) = block {
                println!("🔄 Analyzing Block: {:?}", block.number.unwrap());

                for tx in block.transactions {
                    analyze_transaction(tx, &tx_history).await;
                }
            }
        }

        //  5 seconds before fetching the next block
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

//  MEV patterns
async fn analyze_transaction(tx: Transaction, tx_history: &Arc<Mutex<HashMap<H256, (u64, u128)>>>) {
    let mut history = tx_history.lock().await;
    let hash = tx.hash;
    let gas_price = tx.gas_price.unwrap_or_default().as_u128(); 
    let block_num = tx.block_number.unwrap_or_default().as_u64();
    
    if let Some(to) = tx.to {
        let input_data = hex::encode(&tx.input);

        println!(
            "🔍 TX: Hash={} | To={} | Gas Price={} gwei | Input={}",
            hash, to, gas_price, input_data
        );

        // if interacting with a DEX
        let dex_addresses = vec![
            "0x1111111111111111111111111111111111111111".to_string(), // Replace with real Base DEX contract
        ];

        if dex_addresses.contains(&format!("{:?}", to)) {
            detect_sandwich_attack(&tx, &mut history, block_num, gas_price);
            detect_front_running(&tx, &mut history, block_num, gas_price);
        }
    }

    history.insert(hash, (block_num, gas_price));
}

// ⚠️ Detect Sandwich Attacks
fn detect_sandwich_attack(tx: &Transaction, history: &mut HashMap<H256, (u64, u128)>, block_num: u64, gas_price: u128) {
    for (_, (prev_block, prev_gas)) in history.iter() {
        if *prev_block == block_num && *prev_gas < gas_price {
            println!("⚠️ Possible MEV Attack Detected: Sandwich Attack on TX {}", tx.hash);
        }
    }
}

// ⚠️ Detect Front-Running Bots
fn detect_front_running(tx: &Transaction, history: &mut HashMap<H256, (u64, u128)>, block_num: u64, gas_price: u128) {
    for (_, (prev_block, prev_gas)) in history.iter() {
        if *prev_block == block_num && *prev_gas < gas_price {
            println!("⚠️ Possible MEV Attack Detected: Front-Running on TX {}", tx.hash);
        }
    }
}
