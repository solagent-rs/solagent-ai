use reqwest::Client;
use serde::{Deserialize, Serialize};

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

pub async fn parse_user_input(input: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let api_key = "your-openai-api-key";
    let url = "https://api.openai.com/v1/completions";

    let request = OpenAIRequest {
        prompt: format!("Convert this to a SQL query: {}", input),
        max_tokens: 50,
    };

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?
        .json::<OpenAIResponse>()
        .await?;

    Ok(response.choices[0].text.trim().to_string())
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
