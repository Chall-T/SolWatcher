use anyhow::Result;
use dotenv::dotenv;
use futures::prelude::*;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    rpc_response::{Response, RpcLogsResponse},
};
use solana_commitment_config::CommitmentConfig;
use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

pub mod pool;
pub mod rpc;
pub mod utils;

#[derive(Clone)]
struct PoolWatcher {
    label: &'static str,
    program_id: String,
    log_instruction: String,
    layout: pool::PoolLayout,
}

struct FetchJob {
    signature: String,
    watcher: PoolWatcher,
}

struct Configuration {
    wss_url: String,
    https_url: String,
    watchers: Vec<PoolWatcher>,
    fetch_queue_capacity: usize,
}

impl Configuration {
    fn new() -> Self {
        let wss_url = std::env::var("SOL_WSS")
            .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string());
        let https_url = std::env::var("SOL_HTTPS")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        let fetch_queue_capacity = std::env::var("RPC_FETCH_QUEUE_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(256);

        let mut watchers = Vec::new();
        if env_flag("WATCH_RAYDIUM_AMM", true) {
            watchers.push(PoolWatcher {
                label: "AMM v4",
                program_id: std::env::var("RAYDIUM_LPV4")
                    .unwrap_or_else(|_| "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string()),
                log_instruction: std::env::var("LOG_INSTRUCTION")
                    .unwrap_or_else(|_| "initialize2".to_string()),
                layout: pool::PoolLayout::AmmV4,
            });
        }
        if env_flag("WATCH_RAYDIUM_CPMM", false) {
            watchers.push(PoolWatcher {
                label: "CPMM",
                program_id: std::env::var("RAYDIUM_CPMM")
                    .unwrap_or_else(|_| "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C".to_string()),
                log_instruction: std::env::var("CPMM_LOG_INSTRUCTION")
                    .unwrap_or_else(|_| "Instruction: Initialize".to_string()),
                layout: pool::PoolLayout::Cpmm,
            });
        }

        Configuration {
            wss_url,
            https_url,
            watchers,
            fetch_queue_capacity,
        }
    }
}

fn env_flag(key: &str, default: bool) -> bool {
    match std::env::var(key) {
        Ok(value) => matches!(value.to_lowercase().as_str(), "1" | "true" | "yes" | "on"),
        Err(_) => default,
    }
}

static CONFIG: LazyLock<Configuration> = LazyLock::new(Configuration::new);

static FETCH_TX: LazyLock<mpsc::Sender<FetchJob>> = LazyLock::new(|| {
    let (tx, mut rx) = mpsc::channel::<FetchJob>(CONFIG.fetch_queue_capacity);
    tokio::spawn(async move {
        while let Some(job) = rx.recv().await {
            get_tokens(&job.signature, &job.watcher).await;
        }
    });
    tx
});

const MAX_SEEN_SIGNATURES: usize = 10_000;

static SEEN_SIGNATURES: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

fn mark_signature_seen(program_id: &str, signature: &str) -> bool {
    let key = format!("{program_id}:{signature}");
    let mut seen = SEEN_SIGNATURES
        .lock()
        .expect("signature cache lock poisoned");
    if seen.contains(&key) {
        return false;
    }
    if seen.len() >= MAX_SEEN_SIGNATURES {
        seen.clear();
    }
    seen.insert(key);
    true
}

async fn start_subscriber(watcher: PoolWatcher) -> Result<()> {
    let mut attempts = 0;
    loop {
        attempts += 1;
        let ws_client_result = PubsubClient::new(CONFIG.wss_url.as_str()).await;

        match ws_client_result {
            Ok(ws_client) => {
                if attempts == 1 {
                    println!("[{}] Successfully connected to WebSocket.", watcher.label);
                } else {
                    println!(
                        "[{}] Successfully connected to WebSocket after {} attempts.",
                        watcher.label, attempts
                    );
                }
                attempts = 0;
                let (mut stream, _) = ws_client
                    .logs_subscribe(
                        RpcTransactionLogsFilter::Mentions(vec![watcher.program_id.clone()]),
                        RpcTransactionLogsConfig {
                            commitment: Some(CommitmentConfig::finalized()),
                        },
                    )
                    .await?;

                println!(
                    "[{}] Subscribed to Raydium program {}.",
                    watcher.label, watcher.program_id
                );

                loop {
                    match stream.next().await {
                        Some(response) => {
                            process_message(response, &watcher).await;
                        }
                        None => {
                            eprintln!(
                                "[{}] Stream closed, attempting to reconnect...",
                                watcher.label
                            );
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "[{}] Failed to connect to WebSocket. Attempt {} of 10. Error: {}",
                    watcher.label, attempts, e
                );
                if attempts >= 10 {
                    eprintln!(
                        "[{}] Max reconnection attempts reached. Exiting...",
                        watcher.label
                    );
                    return Err(anyhow::anyhow!(
                        "[{}] Max reconnection attempts reached",
                        watcher.label
                    ));
                }
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let _ = &*rpc::RPC;
    let _ = &*FETCH_TX;

    let logger = utils::Logger::new("Setup");

    if CONFIG.watchers.is_empty() {
        return Err(anyhow::anyhow!(
            "No pool watchers enabled. Set WATCH_RAYDIUM_AMM and/or WATCH_RAYDIUM_CPMM to true."
        ));
    }

    logger.log(&format!("Solana RPC websocket: {:?}", CONFIG.wss_url));
    logger.log(&format!("Solana RPC http: {:?}", CONFIG.https_url));
    for watcher in &CONFIG.watchers {
        logger.log(&format!(
            "Watcher {:?}: program={}, log_pattern={:?}",
            watcher.label, watcher.program_id, watcher.log_instruction
        ));
    }

    let mut tasks = Vec::new();
    for watcher in CONFIG.watchers.clone() {
        tasks.push(tokio::spawn(async move { start_subscriber(watcher).await }));
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}

async fn process_message(response: Response<RpcLogsResponse>, watcher: &PoolWatcher) {
    let value = response.value;
    let matches_instruction = value
        .logs
        .iter()
        .any(|log| pool::log_matches_instruction(log, &watcher.log_instruction, watcher.layout));
    if !matches_instruction {
        return;
    }
    if !mark_signature_seen(&watcher.program_id, &value.signature) {
        return;
    }

    let job = FetchJob {
        signature: value.signature.clone(),
        watcher: watcher.clone(),
    };
    if let Err(error) = FETCH_TX.try_send(job) {
        let logger = utils::Logger::new(format!("{} handler", watcher.label));
        logger.error(&format!(
            "Fetch queue full, dropped {}: {}",
            value.signature, error
        ));
    }
}

async fn get_tokens(sign: &str, watcher: &PoolWatcher) {
    let logger = utils::Logger::new(format!("{} handler", watcher.label));
    let tx = match rpc::RPC.get_transaction(sign, "jsonParsed").await {
        Ok(tx) => tx,
        Err(e) => {
            logger.error(&format!("Failed to retrieve transaction {sign}: {e}"));
            return;
        }
    };

    let instructions = pool::matching_instructions(&tx, &watcher.program_id);
    if !instructions.is_empty() {
        pool::token_show_info(instructions, watcher.layout, watcher.label);
    }
}
