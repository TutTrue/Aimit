use serde::{Deserialize, Serialize};
use std::fs;

use crate::models::ModelType;

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    default_model: ModelType,
    prompt: String,
    api_keys: ApiKeysConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiKeysConfig {
    gemini_api_key: Option<String>,
    deepseek_api_key: Option<String>,
}

const CONFIG_FILE: &str = "Config.toml";

impl Settings {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let settings = fs::read_to_string(CONFIG_FILE)?;
        let settings: Settings = toml::from_str(&settings)?;
        Ok(settings)
    }

    pub fn get_prompt(&self) -> &str {
        &self.prompt
    }

    pub fn get_default_model(&self) -> &ModelType {
        &self.default_model
    }

    pub fn update_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }

    pub fn update_api_key(&mut self, service: ModelType, api_key: Option<String>) {
        match service {
            ModelType::GEMINI => self.api_keys.gemini_api_key = api_key,
        }
    }

    pub fn update_default_model(&mut self, model: ModelType) {
        self.default_model = model;
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings = toml::to_string(self)?;
        fs::write(CONFIG_FILE, settings)?;
        Ok(())
    }

    pub fn get_api_key(&self, service: ModelType) -> Option<&str> {
        match service {
            ModelType::GEMINI => self.api_keys.gemini_api_key.as_deref(),
        }
    }
}
