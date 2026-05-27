use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::LazyLock;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::utils::Logger;

const MAX_TX_FETCH_RETRIES: u32 = 30;
const MAX_RATE_LIMIT_RETRIES: u32 = 8;

pub static RPC: LazyLock<RpcClient> = LazyLock::new(RpcClient::from_env);

pub struct RpcClient {
    http: Client,
    headers: HeaderMap,
    url: String,
    min_interval: Duration,
    last_request: Mutex<Instant>,
}

impl RpcClient {
    pub fn from_env() -> Self {
        let min_interval_ms = std::env::var("RPC_MIN_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(250);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        RpcClient {
            url: std::env::var("SOL_HTTPS")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
            http: Client::new(),
            headers,
            min_interval: Duration::from_millis(min_interval_ms),
            last_request: Mutex::new(Instant::now() - Duration::from_secs(1)),
        }
    }

    pub async fn get_transaction(&self, signature: &str, encoding: &str) -> Result<Value, String> {
        let logger = Logger::new("RPC");
        let mut null_attempts = 0u32;
        let mut rate_limit_attempts = 0u32;

        loop {
            self.throttle().await;

            let body = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getTransaction",
                "params": [
                    signature,
                    {
                        "encoding": encoding,
                        "maxSupportedTransactionVersion": 0
                    }
                ]
            });

            let response = self
                .http
                .post(&self.url)
                .headers(self.headers.clone())
                .json(&body)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            let body_json: Value = response.json().await.map_err(|e| e.to_string())?;

            if let Some(error) = body_json.get("error") {
                if is_rate_limited(error) {
                    rate_limit_attempts += 1;
                    if rate_limit_attempts >= MAX_RATE_LIMIT_RETRIES {
                        return Err(format!(
                            "rate limited fetching {signature} after {MAX_RATE_LIMIT_RETRIES} retries"
                        ));
                    }
                    let backoff = Duration::from_millis(500 * 2u64.pow(rate_limit_attempts - 1));
                    logger.debug(&format!(
                        "RPC 429, backing off {:?} ({rate_limit_attempts}/{MAX_RATE_LIMIT_RETRIES})",
                        backoff
                    ));
                    sleep(backoff).await;
                    continue;
                }
                return Err(format!("RPC error for {signature}: {error}"));
            }

            rate_limit_attempts = 0;

            if body_json["result"].is_null() {
                null_attempts += 1;
                if null_attempts >= MAX_TX_FETCH_RETRIES {
                    return Err(format!(
                        "transaction {signature} not available after {MAX_TX_FETCH_RETRIES} attempts"
                    ));
                }
                logger.debug(&format!(
                    "Transaction not ready, retry {null_attempts}/{MAX_TX_FETCH_RETRIES} for \"{signature}\""
                ));
                sleep(Duration::from_secs(1)).await;
                continue;
            }

            return Ok(body_json);
        }
    }

    async fn throttle(&self) {
        let mut last = self.last_request.lock().await;
        let elapsed = last.elapsed();
        if elapsed < self.min_interval {
            sleep(self.min_interval - elapsed).await;
        }
        *last = Instant::now();
    }
}

fn is_rate_limited(error: &Value) -> bool {
    error.get("code").and_then(|c| c.as_i64()) == Some(429)
}
