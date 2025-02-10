use ethers::prelude::*;
use std::env;
use dotenv::dotenv;
use eyre::Result;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let ws_url = env::var("WS_RPC_URL").expect("WS_RPC_URL must be set");

    //  WebSocket
    let provider = Provider::<Ws>::connect(ws_url).await?;
    println!("✅ Connected to Base Mainnet WebSocket!");

    // new blocks
    let mut stream = provider.subscribe_blocks().await?;

    println!("🔍 Listening for new blocks...");

    while let Some(block) = stream.next().await {
        if let Some(block_hash) = block.hash {
            println!("🚀 New Block: {:?}, Number: {:?}", block_hash, block.number);
            
            // block details
            if let Ok(full_block) = provider.get_block(block_hash).await {
                if let Some(block) = full_block {
                    println!("📦 Transactions in Block {}: {:?}", block.number.unwrap(), block.transactions);
                }
            }
        }
    }

    Ok(())
}
