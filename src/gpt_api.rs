
pub fn generate_commit_message(diff: &str) -> String {
    let api_key = "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"; // replace with your key
    let endpoint = "https://api.openai.com/v1/chat/completions"; // this might differ based on your OpenAI version and specifics

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
        .send()
        .expect("Failed to send request");

    let response_data:serde_json::Value = response.json().expect("Failed to parse response");
    let content = response_data["choices"][0]["message"]["content"].to_string();

    escape_special_characters(content).trim().to_string()
}

fn escape_special_characters(mut input: String) -> String {
    input = input.replace("\"", "\\\"");
    input = input.replace("'", "\\'");
    input
}
