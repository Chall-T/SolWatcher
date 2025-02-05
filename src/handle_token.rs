use async_recursion::async_recursion;
use async_std::task;
use reqwest::header;
use reqwest::Client;
use serde_json::Value;
use std::any::Any;
use std::any::TypeId;
use std::error::Error;
use std::time::Duration;

use crate::utils::Logger;

#[async_recursion]
pub async fn get_transaction(
    signature: &str,
    encoding: &str,
    http: &str,
) -> Result<Value, Box<dyn Error>> {
    let logger = Logger::new(String::from("Token handler"));
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = Client::new();
    let json_data = format!(
        "
    {{
        \"jsonrpc\": \"2.0\",
        \"id\": 1,
        \"method\": \"getTransaction\",
        \"params\": [
            \"{signature}\",
            {{
                \"encoding\": \"{encoding}\",
                \"maxSupportedTransactionVersion\": 0
            }}
        ]
    }}"
    );
    let response = client
        .post(http)
        .headers(headers)
        .body(json_data)
        .send()
        .await?;

    let body = response.text().await?;
    
    let mut body_json: Value = serde_json::from_str(body.as_str()).expect("Failed to parse JSON");
    logger.debug(format!("getTransaction [\"result\"] type: {:?}, is type string: {:?}", body_json["result"].type_id(), body_json["result"].type_id() == TypeId::of::<String>()));

    if body_json["result"].type_id() == TypeId::of::<String>() {
        
        logger.debug(format!("Resending getTransaction request for \"{}\" signature", signature));
        task::sleep(Duration::from_secs(1)).await;
        body_json = get_transaction(signature, encoding, http).await.unwrap();
    }

    Ok(body_json)
}

pub fn get_instructions(json: Value) -> Vec<serde_json::Value> {
    let mut instructions = Vec::new();
    if let Some(raw_instructions) =
        json["result"]["transaction"]["message"]["instructions"].as_array()
    {
        for raw_instruction in raw_instructions {
            instructions.push(raw_instruction.clone());
        }
    }
    instructions
}

pub fn get_instructions_with_program_id(json: Value, program_id: String) -> Vec<serde_json::Value> {
    let mut filtred_instuctions = Vec::new();

    let instructions = get_instructions(json);
    if instructions.is_empty() {
        let logger = Logger::new(String::from("Token handler"));
        logger.error("No instructions found".to_string());
        return filtred_instuctions;
    }
    for instruction in instructions {
        if instruction["programId"].eq(&program_id) {
            filtred_instuctions.push(instruction);
        }
    }
    filtred_instuctions
}
pub fn get_tokens_info(
    instruction: serde_json::Value,
) -> (serde_json::Value, serde_json::Value, serde_json::Value) {
    let accounts = &instruction["accounts"];
    let pair = &accounts[4];
    let token0 = &accounts[8];
    let token1 = &accounts[9];
    (token0.clone(), token1.clone(), pair.clone())
}

pub fn token_show_info_detailed(instructions: Vec<serde_json::Value>){
    for instruction in instructions {
        let tokens = get_tokens_info(instruction);
        println!("Token0: {}", &tokens.0.as_str().unwrap().replace('\"', ""));
        println!("Token1: {}", &tokens.1.as_str().unwrap().replace('\"', ""));
        println!(
            "LP Pair: {}",
            &tokens.2.as_str().unwrap().replace('\"', "")
        );
        println!(
            "Dex: https://dexscreener.com/solana/{}",
            &tokens.2.as_str().unwrap().replace('\"', "")
        );
    }
}
pub fn token_show_info(instructions: Vec<serde_json::Value>){
    let logger = Logger::new(String::from("Token handler"));
    for instruction in instructions {
        let tokens = get_tokens_info(instruction);
        let token: &str;
        if "So11111111111111111111111111111111111111112" == tokens.0.as_str().unwrap(){
            token = &tokens.1.as_str().unwrap();
        }else{
            token = &tokens.2.as_str().unwrap();
        }
        let lp_pair = &tokens.2.as_str().unwrap();
        logger.log(format!("new pair found (Token: {} LP Pair: {})", token, lp_pair));
    }
}