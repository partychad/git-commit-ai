
use std::env;


pub fn generate_commit_message(diff: &str) -> String {
    let api_key = get_api_key("GPT_API_KEY".to_string());
    let endpoint = "https://api.openai.com/v1/chat/completions";

    let client = reqwest::blocking::Client::new();

    let response = client.post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [
            {
                "role": "user",
                "content": format!("Based on the following git diff, suggest a formatted and structured but succinct commit message. Only use \" for quoting\n{}", diff)
            }
            ],
            "temperature": 0.7
        }))
        .send()
        .expect("Failed to send request");

    let response_data:serde_json::Value = response.json().expect("Failed to parse response");
    let content = response_data["choices"][0]["message"]["content"].to_string();

    format!("{:?}",escape_special_characters(content))
}


pub fn get_api_key(variable_name:String) -> String {
    let api_key = match env::var(variable_name) {
        Ok(key) => key,
        Err(_) => panic!("API key not found in environment variables!")
    };
    api_key
}

fn escape_special_characters(mut input: String) -> String {
    input = input.replace("\"", "\\\"");
    input = input.replace("'", "\\'");
    input = input.replace("`", "\\`");

    input
}