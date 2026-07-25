use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SolanaPayArgs {
    pub recipient: String,
    pub amount: String,
    pub spl_mint: Option<String>,
    pub label: String,
}

#[derive(Serialize, Debug)]
pub struct ToolResult {
    pub status: String,
    pub solana_url: String,
    pub message: String,
}

pub trait RuntimeContext {
    fn read_config(&self, key: &str) -> String;
    fn log_record(&self, level: &str, msg: &str);
}

pub fn handle_name() -> String {
    "solana-pay-request".to_string()
}

pub fn handle_description() -> String {
    "Generates a standard base58 Solana Pay transaction request string and context.".to_string()
}

pub fn handle_parameters_schema() -> String {
    r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "recipient": { "type": "string" },
            "amount": { "type": "string" },
            "spl_mint": { "type": "string" },
            "label": { "type": "string" }
        },
        "required": ["recipient", "amount", "label"]
    }"#
    .to_string()
}

pub fn handle_execute<C: RuntimeContext>(args_json: &str, ctx: &C) -> Result<String, String> {
    let args: SolanaPayArgs = serde_json::from_str(args_json)
        .map_err(|e| format!("Invalid arguments structure: {}", e))?;

    ctx.log_record("info", &format!("Processing payment request for amount: {}", args.amount));

    let allowed_mints_raw = ctx.read_config("allowed_mints");
    if let Some(ref mint) = args.spl_mint {
        if !allowed_mints_raw.contains(mint) {
            ctx.log_record("error", "Security Guardrail Triggered: Token mint is unauthorized.");
            return Err("Execution Denied: The requested token mint is not whitelisted by the node operator.".to_string());
        }
    }

    let base_url = format!(
        "solana:{}?amount={}&label={}",
        args.recipient,
        args.amount,
        urlencoding::encode(&args.label)
    );
    
    let final_url = match args.spl_mint {
        Some(mint) => format!("{}&spl-token={}", base_url, mint),
        None => base_url,
    };

    let response = ToolResult {
        status: "success".to_string(),
        solana_url: final_url,
        message: format!("Created valid T1 payment request destination for {} native units.", args.amount),
    };

    serde_json::to_string(&response).map_err(|e| e.to_string())
}