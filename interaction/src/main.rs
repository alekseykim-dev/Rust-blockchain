use ethers::prelude::*;
use ethers::abi::Abi;
use std::sync::Arc;
use serde_json;
use std::fs;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://base-mainnet.infura.io/v3/62e7e0645e5a4f219ebfd6c760b29497";

    let contract_address: Address = "0x26A54955a5fb9472D3eDFeAc9B8E4c0ab5779eD3"
        .parse()
        .expect("Invalid contract address format");

    // Load ABI from a file
    let abi: Abi = serde_json::from_str(
        &fs::read_to_string("contract_abi.json").expect("Failed to read ABI file"),
    )
    .expect("Failed to parse ABI JSON");

    let provider = Provider::<Http>::try_from(rpc_url).expect("Invalid RPC URL");
    let client = Arc::new(provider);

    let contract = Contract::new(contract_address, abi, client);

    let value: U256 = contract
        .method::<_, U256>("getValue", ())
        .expect("Function not found in ABI")
        .call()
        .await
        .expect("Call to contract failed");

    println!("Contract value: {}", value);

    Ok(())
}
