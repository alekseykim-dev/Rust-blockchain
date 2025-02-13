use reqwest;
use serde_json::Value;
use tokio;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum,usd-coin&vs_currencies=usd";
    
    let response = reqwest::get(url).await?;
    let body: Value = response.json().await?;

    let eth_price = body["ethereum"]["usd"].as_f64().unwrap_or(0.0);
    let usdc_price = body["usd-coin"]["usd"].as_f64().unwrap_or(0.0);

    println!("💰 ETH Price: ${}", eth_price);
    println!("💵 USDC Price: ${}", usdc_price);

    Ok(())
}
