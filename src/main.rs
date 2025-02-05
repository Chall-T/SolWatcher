use anyhow::Result;
use dotenv::dotenv;
use futures::prelude::*;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    rpc_response::{Response, RpcLogsResponse},
};
use solana_sdk::commitment_config::CommitmentConfig;
use tokio::time::sleep;
use std::time::Duration;
pub mod handle_token;
pub mod utils;

struct Configuration {
    wss_url: String,
    https_url: String,
    log_instruction: String,
    raydium_lpv4: String,
}
impl Configuration {
    fn new() -> Self {
        let wss_url = std::env::var("SOL_WSS")
            .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string());
        let https_url = std::env::var("SOL_HTTPS")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        let log_instruction =
            std::env::var("LOG_INSTRUCTION").unwrap_or_else(|_| "initialize2".to_string());
        let raydium_lpv4 = std::env::var("RAYDIUM_LPV4")
            .unwrap_or_else(|_| "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string());

        Configuration {
            wss_url,
            https_url,
            log_instruction,
            raydium_lpv4,
        }
    }
}
lazy_static::lazy_static! {
    static ref CONFIG: Configuration = Configuration::new();
}



async fn start_subscriber() -> Result<()> {
    let mut attempts = 0;
    loop {
        attempts += 1;
        let ws_client_result = PubsubClient::new(CONFIG.wss_url.as_str()).await;

        match ws_client_result {
            Ok(ws_client) => {
                if attempts == 1{
                    println!("Successfully connected to WebSocket.");
                }else{
                    println!("Successfully connected to WebSocket after {} attempts.", attempts);
                }
                attempts = 0;
                let (mut stream, _) = ws_client
                    .logs_subscribe(
                        RpcTransactionLogsFilter::Mentions(vec![CONFIG.raydium_lpv4.to_string()]),
                        RpcTransactionLogsConfig {
                            commitment: Some(CommitmentConfig::finalized()),
                        },
                    )
                    .await?;

                println!("Subscribed to Raydium Liquidity Pool.");

                loop {
                    match stream.next().await {
                        Some(response) => {
                            process_message(response).await;
                        }
                        None => {
                            eprintln!("Stream closed, attempting to reconnect...");
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to WebSocket. Attempt {} of 10. Error: {}", attempts, e);
                if attempts >= 10 {
                    eprintln!("Max reconnection attempts reached. Exiting...");
                    return Err(anyhow::anyhow!("Max reconnection attempts reached").into());
                }
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let logger = utils::Logger::new("Setup".to_string());

    logger.log(format!("Solana RPC websocket: {:?}", CONFIG.wss_url.as_str()));
    logger.log(format!("Solana RPC http: {:?}", CONFIG.https_url.as_str()));
    logger.log(format!("Log instruction: {:?}", CONFIG.log_instruction.as_str()));

    start_subscriber().await
}
async fn process_message(response: Response<RpcLogsResponse>) {
    let value = response.value;
    for log in value.logs {
        if !log.contains(CONFIG.log_instruction.as_str()) {
            continue;
        }
        let signature_str = &value.signature;
        get_tokens(&signature_str, CONFIG.raydium_lpv4.to_string()).await;
    }
}
async fn get_tokens(sign: &str, program: String) {
    let result = handle_token::get_transaction(sign, "jsonParsed", CONFIG.https_url.as_str())
        .await
        .expect("Failed to retrieve transaction data. Check the network or RPC server.");

    let instructions = handle_token::get_instructions_with_program_id(result, program);
    
    
    handle_token::token_show_info(instructions);

}
