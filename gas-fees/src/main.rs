use ethers::prelude::*;
use std::env;
use dotenv::dotenv;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let http_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let provider = Provider::<Http>::try_from(http_url)?;

    //  current gas price
    let gas_price = provider.get_gas_price().await?;
    println!("⚡ Current Gas Price: {} gwei", ethers::utils::format_units(gas_price, 9)?);

    //  gas for a sample transaction
    let from: Address = "0xf7757fF100FC0A1932EC9f5D5CD2A91ff423a2A0".parse()?;  
    let to: Address = "0xdfde9957bba11960662a8e89d1f05784d5937e2f".parse()?;  

    let tx = TransactionRequest::new()
        .from(from)
        .to(to)
        .value(ethers::utils::parse_ether("0.001")?) // 0.001 eth
        .into();

    let gas_estimate = provider.estimate_gas(&tx, None).await?;
    println!("⛽ Estimated Gas: {} units", gas_estimate);

    // total transaction fee in ETH
    let total_fee = gas_price * gas_estimate;
    let total_fee_eth = ethers::utils::format_units(total_fee, 18)?; // Convert from Wei to ETH

    println!("💰 Total Estimated Fee: {} ETH", total_fee_eth);

    Ok(())
}
