use crate::{config::settings, error::error::AimitError};

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
    ) -> Result<String, AimitError> {
        let client = reqwest::Client::new();
        let settings = settings::Settings::new().unwrap();

        let request = serde_json::json!({
            "contents": [{
                "parts": [{
                  "text": format!("{}{}",settings.get_prompt(), diff)
                }]
              }]
        });

        let response: Response = client
            .post(&format!(
                "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
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
