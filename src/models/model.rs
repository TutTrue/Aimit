use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::error::error::AimitError;

#[async_trait]
pub trait AiModel {
    async fn generate_commit_message(
        &self,
        diff: &str,
    ) -> Result<String, AimitError>;
}

#[derive(Debug, Deserialize, Serialize, Clone, EnumString)]
pub enum ModelType {
    GEMINI,
}

pub struct ModelFactory;

impl ModelFactory {
    pub fn create_model(model_type: ModelType, key: String) -> Box<dyn AiModel> {
        match model_type {
            ModelType::GEMINI => Box::new(super::gemini::GeminiModel::new(key)),
        }
    }
}
