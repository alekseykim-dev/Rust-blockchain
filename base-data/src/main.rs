use eyre::Result;
use reqwest;
use serde_json::Value;
use dotenv::dotenv;
use std::env;

const USDC_ADDRESS: &str = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913";

async fn get_transactions(api_key: &str, address: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let mut tx_number = 1;

    // ETH transactions
    let eth_url = format!(
        "https://api.basescan.org/api?module=account&action=txlist&address={}&sort=desc&apikey={}",
        address, api_key
    );
    let eth_data: Value = client.get(&eth_url).send().await?.json().await?;

    // USDC transfers
    let usdc_url = format!(
        "https://api.basescan.org/api?module=account&action=tokentx&contractaddress={}&address={}&sort=desc&apikey={}",
        USDC_ADDRESS, address, api_key
    );
    let usdc_data: Value = client.get(&usdc_url).send().await?.json().await?;

    println!("ETH Transactions:");
    if let Some(txs) = eth_data["result"].as_array() {
        for tx in txs {
            let value_eth = (tx["value"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0)) / 1e18;
            if value_eth > 0.0 {
                println!("#{} | Block {} | {} ETH | From: {} | To: {}", 
                    tx_number,
                    tx["blockNumber"].as_str().unwrap_or("unknown"),
                    value_eth,
                    tx["from"].as_str().unwrap_or("unknown"),
                    tx["to"].as_str().unwrap_or("unknown")
                );
                tx_number += 1;
            }
        }
    }

    println!("\nUSDC Transfers:");
    if let Some(transfers) = usdc_data["result"].as_array() {
        for tx in transfers {
            let value_usdc = (tx["value"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0)) / 1_000_000.0;
            println!("#{} | Block {} | {} USDC | From: {} | To: {}", 
                tx_number,
                tx["blockNumber"].as_str().unwrap_or("unknown"),
                value_usdc,
                tx["from"].as_str().unwrap_or("unknown"),
                tx["to"].as_str().unwrap_or("unknown")
            );
            tx_number += 1;
        }
    }

    println!("\nTotal transactions: {}", tx_number - 1);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // variables from .env file
    dotenv().ok();
    
    //  API key
    let api_key = env::var("BASESCAN_API_KEY")
        .expect("BASESCAN_API_KEY must be set in .env file");
        
    let address = "0xf7757fF100FC0A1932EC9f5D5CD2A91ff423a2A0";
    
    println!("Fetching transactions for: {}\n", address);
    get_transactions(&api_key, address).await?;
    
    Ok(())
}