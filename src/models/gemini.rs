use super::model::AiModel;

use async_trait::async_trait;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
struct ResponsePart {
    text: String,
}

#[derive(Deserialize, Debug)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize, Debug)]
struct Response {
    candidates: Vec<Candidate>,
}

pub struct GeminiModel {
    api_key: String,
}

impl GeminiModel {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl AiModel for GeminiModel {
    async fn generate_commit_message(
        &self,
        diff: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let request = serde_json::json!({
            "contents": [{
                "parts": [{
                  "text": format!(r#"Analyze the following git diff and generate a concise and meaningful commit message summarizing the changes.
The commit message should follow best practices, including a short title and an optional detailed description if necessary.
git diff:
{}
Requirements:
  Title: 50 characters or less, summarizing the change.
  Optional Description: If the change requires context, provide a brief explanation in the body."#, diff)
                }]
              }]
        });

        let response: Response = client
            .post(&format!(
                "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
                self.api_key
            ))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.candidates[0].content.parts[0].text.clone())
    }
}
