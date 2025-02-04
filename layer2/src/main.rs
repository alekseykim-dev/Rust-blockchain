use ethers::prelude::*;
use ethers::abi::Abi;
use std::convert::TryFrom;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Ethereum node (Infura)
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set in .env");

    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    let provider = Arc::new(provider);

    // current block number
    let block_number = provider.get_block_number().await?;
    println!("Current block number: {}", block_number);

    // wallet address
    let wallet_address: Address = "0xf7757fF100FC0A1932EC9f5D5CD2A91ff423a2A0".parse()?;
    // Fetch ETH balance

    let eth_balance = provider.get_balance(wallet_address, None).await?;
    println!("Wallet Balance: {} ETH", ethers::utils::format_ether(eth_balance));

    // USDC Contract Address
    let usdc_contract_address: Address = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".parse()?; // Confirm this address
    // ERC-20 ABI 
    let abi: Abi = serde_json::from_str(r#"[
      {
        "constant": true,
        "inputs": [{"name": "_owner", "type": "address"}],
        "name": "balanceOf",
        "outputs": [{"name": "balance", "type": "uint256"}],
        "type": "function"
      }
    ]"#)?;

    // Interact with the USDC contract
    let usdc_contract = Contract::new(usdc_contract_address, abi, provider.clone());

    // Error handling for balanceOf call
    match usdc_contract.method::<Address, U256>("balanceOf", wallet_address)?.call().await {
        Ok(usdc_balance) => {
        
            let usdc_balance_formatted = usdc_balance.as_u128() as f64 / 1_000_000.0;
            println!("Wallet Balance: {:.6} USDC", usdc_balance_formatted);
        }
        Err(e) => {
            eprintln!("Failed to fetch USDC balance: {:?}", e);
        }
    }
    Ok(())
}