use serde_json::Value;

use crate::rpc::RPC;
use crate::utils::Logger;

pub async fn get_transaction(
    signature: &str,
    encoding: &str,
    _http: &str,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    RPC.get_transaction(signature, encoding).await
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

pub fn get_instructions_with_program_id(json: Value, program_id: &str) -> Vec<serde_json::Value> {
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
