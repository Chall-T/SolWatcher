use serde_json::Value;

use crate::utils::Logger;

const WSOL_MINT: &str = "So11111111111111111111111111111111111111112";

#[derive(Clone, Copy, Debug)]
pub enum PoolLayout {
    AmmV4,
    Cpmm,
}

impl PoolLayout {
    fn account_indices(self) -> (usize, usize, usize) {
        match self {
            // (token0, token1, pool)
            PoolLayout::AmmV4 => (8, 9, 4),
            PoolLayout::Cpmm => (4, 5, 3),
        }
    }
}

pub fn log_matches_instruction(log: &str, pattern: &str, layout: PoolLayout) -> bool {
    match layout {
        PoolLayout::Cpmm => log.ends_with(pattern),
        PoolLayout::AmmV4 => log.contains(pattern),
    }
}

pub fn matching_instructions(tx: &Value, program_id: &str) -> Vec<Value> {
    let Some(instructions) = tx["result"]["transaction"]["message"]["instructions"].as_array()
    else {
        return Vec::new();
    };
    instructions
        .iter()
        .filter(|ix| ix["programId"].as_str() == Some(program_id))
        .cloned()
        .collect()
}

pub fn get_pool_tokens_info(
    instruction: Value,
    layout: PoolLayout,
) -> Option<(String, String, String)> {
    let accounts = instruction["accounts"].as_array()?;
    let (token0_idx, token1_idx, pool_idx) = layout.account_indices();
    let token0 = account_pubkey(accounts.get(token0_idx)?)?;
    let token1 = account_pubkey(accounts.get(token1_idx)?)?;
    let pool = account_pubkey(accounts.get(pool_idx)?)?;
    Some((token0, token1, pool))
}

fn account_pubkey(account: &Value) -> Option<String> {
    if let Some(pk) = account.as_str() {
        return Some(pk.to_string());
    }
    account
        .get("pubkey")
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

pub fn token_show_info(instructions: Vec<Value>, layout: PoolLayout, label: &str) {
    let logger = Logger::new(label.to_string());
    for instruction in instructions {
        let Some((token0, token1, pool)) = get_pool_tokens_info(instruction, layout) else {
            logger.error("Could not read pool accounts from instruction");
            continue;
        };
        let token = if token0 == WSOL_MINT {
            &token1
        } else {
            &token0
        };
        logger.log(&format!("new pair found (Token: {token} LP Pair: {pool})"));
    }
}
