
use std::env;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommitMessageError {
    #[error("Failed to send request")]
    RequestError(#[from] reqwest::Error),

    #[error("Failed to parse response")]
    ParseError(#[from] serde_json::Error),

    #[error("API key not found in environment variables!")]
    ApiKeyNotFound,

    #[error("No change made since the last commit!")]
    NoChangeMade,
}


pub fn generate_commit_message(diff: &str) -> Result<String, CommitMessageError> {
    let api_key = get_api_key("GPT_API_KEY".to_string()).unwrap();
    let endpoint = "https://api.openai.com/v1/chat/completions";

    let client = reqwest::blocking::Client::new();

    let response = client.post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [
            {
                "role": "user",
                "content": format!("Based on the following git diff, suggest a formatted and structured but succinct commit message\n{}", diff)
            }
            ],
            "temperature": 0.7
        }))
        .send()?;

    let response_data:serde_json::Value = response.json()?;
    let content = response_data["choices"][0]["message"]["content"].to_string();
    Ok(format!("{}",escape_special_characters(content)))
}


pub fn get_api_key(variable_name:String) -> Result<String, CommitMessageError> {
    let api_key = env::var(variable_name).map_err(|_| CommitMessageError::ApiKeyNotFound)?;
    Ok(api_key)
}

fn escape_special_characters(mut input: String) -> String {
    input = input.replace("\"", "\\\"");
    input = input.replace("'", "\\'");
    input = input.replace("`", "\\`");
    input
}