use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let rpc_url = "https://mainnet.base.org";
    let request_body = json!({
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "params": [],
        "id": 1
    });

    let response = client.post(rpc_url)
        .json(&request_body)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;

    //  converting hex string to decimal
    if let Some(hex_str) = response_json["result"].as_str() {
        if let Ok(block_number) = u64::from_str_radix(hex_str.trim_start_matches("0x"), 16) {
            println!("Latest block number: {} (Hex: {})", block_number, hex_str);
        } else {
            println!("Failed to parse block number.");
        }
    } else {
        println!("Invalid response: {:?}", response_json);
    }

    Ok(())
}
