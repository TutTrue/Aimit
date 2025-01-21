use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait AiModel {
    async fn generate_commit_message(
        &self,
        diff: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ModelType {
    GEMINI,
    DEEPSEEK,
}

pub struct ModelFactory;

impl ModelFactory {
    pub fn create_model(model_type: ModelType, key: String) -> Box<dyn AiModel> {
        match model_type {
            ModelType::GEMINI => Box::new(super::gemini::GeminiModel::new(key)),
            _ => panic!("Model not implemented"),
        }
    }
}
