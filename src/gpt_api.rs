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

pub struct CommitMessageGenerator {
    endpoint: String,
    model: String,
    default_message: String,
    api_key_env_variable: String,
}

impl CommitMessageGenerator {
    pub fn new(
        endpoint: &str,
        model: &str,
        default_message: &str,
        api_key_env_variable: &str,
    ) -> Self {
        CommitMessageGenerator {
            endpoint: endpoint.to_string(),
            model: model.to_string(),
            default_message: default_message.to_string(),
            api_key_env_variable: api_key_env_variable.to_string(),
        }
    }
    pub fn generate_commit_message(
        &self,
        diff: &str,
        untracked_files: &str,
    ) -> Result<String, CommitMessageError> {
        if diff.is_empty()  {
            return Err(CommitMessageError::NoChangeMade);
        }

        let api_key = self.get_api_key(self.api_key_env_variable.to_string())?;
        let endpoint = &self.endpoint;

        let client = reqwest::blocking::Client::new();

        let response = client.post(endpoint)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&serde_json::json!({
            "model": self.model,
            "messages": [
            {
                "role": "user",
                "content": format!("{}\n diff: {} \nuntracked files: {}" ,self.default_message, diff, untracked_files)
            }
            ],
            "temperature": 0.7
        }))
            .send()?;

        let response_data: serde_json::Value = response.json()?;
        let content = response_data["choices"][0]["message"]["content"].to_string();
        Ok(format!("{}", self.escape_special_characters(content)))
    }

    fn get_api_key(&self, variable_name: String) -> Result<String, CommitMessageError> {
        let api_key = env::var(variable_name).map_err(|_| CommitMessageError::ApiKeyNotFound)?;
        Ok(api_key)
    }

    fn escape_special_characters(&self, mut input: String) -> String {
        input = input.replace("'", "");
        input = input.replace("\"", "");
        input
    }

    pub fn display_parameters(&self) {
        println!("Endpoint: {}", self.endpoint);
        println!("Model: {}", self.model);
        println!("Default Message: {} \n", self.default_message);
    }
}
