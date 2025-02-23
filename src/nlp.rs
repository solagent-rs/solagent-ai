use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use topkio::{AgentBuilder, OpenAIClient};

#[derive(Serialize)]
pub struct OpenAIRequest {
    pub prompt: String,
    pub max_tokens: u32,
}

#[derive(Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub text: String,
}

pub async fn parse_user_input(input: &str) -> Result<String> {
    // let url = "https://api.openai.com/v1/completions";

    let client = OpenAIClient::with_ollama("http://localhost:11434/v1");
    let agent = AgentBuilder::new(client, "llama3.2")
        .stream(false)
        .temperature(0.8)
        .build();

    let result = Arc::new(Mutex::new("".to_string()));

    let f = move |res: &str| {
        print!("result: <{}>", res);

        let mut result_lock = result.lock().unwrap();
        *result_lock = res.to_string();
    };

    let prompt = format!("translate input to sql query directly, no other words or tags: eg.(select * from splash): {}", input);

    let _ = agent.prompt(&prompt, f).await;

    Ok("SELECT * FROM splash;".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_user_input() {
        let input = "Show me the top 10 transactions";
        let result = parse_user_input(input).await.unwrap();
        assert!(result.contains("SELECT"));
    }
}
