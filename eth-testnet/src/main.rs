use ethers::prelude::*;
use std::convert::TryFrom;
use dotenv::dotenv;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the .env file
    dotenv().ok();

    // Base testnet endpoint
    let rpc_url = env::var("BASE_RPC_URL").expect("BASE_RPC_URL must be set");

    // Connecting to the Base node
    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    // latest block number
    let block_number = provider.get_block_number().await?;
    println!("Connected to Base Testnet");
    println!("Current Block Number: {}", block_number);

    Ok(())
}
