use home::home_dir;
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

const CONFIG_FILE: &str = ".config/aimit/Config.toml";

impl Settings {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home = home_dir().ok_or("Could not find home directory")?;
        let config_path = home.join(CONFIG_FILE);

        if config_path.exists() {
            let settings = fs::read_to_string(config_path)?;
            let settings: Settings = toml::from_str(&settings)?;
            Ok(settings)
        } else {
            Self::create_default()
        }
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
        let home = home_dir().ok_or("Could not find home directory")?;
        let config_path = home.join(CONFIG_FILE);

        fs::write(config_path, settings)?;
        Ok(())
    }

    pub fn get_api_key(&self, service: ModelType) -> Option<&str> {
        match service {
            ModelType::GEMINI => self.api_keys.gemini_api_key.as_deref(),
        }
    }

    fn create_default() -> Result<Self, Box<dyn std::error::Error>> {
        let home = home_dir().ok_or("Could not find home directory")?;
        let config_path = home.join(CONFIG_FILE);

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let settings = Settings {
            default_model: ModelType::GEMINI,
            prompt: r#"Analyze the following git diff and generate a concise and meaningful commit message summarizing the changes.
The commit message should follow best practices, including a short title and an optional detailed description if necessary.
Requirements:
  Title: 50 characters or less, summarizing the change.
  Optional Description: If the change requires context, provide a brief explanation in the body.

git diff:

"#.to_string(),
            api_keys: ApiKeysConfig {
                gemini_api_key: Some("".to_string()),
                deepseek_api_key: Some("".to_string()),
            },
        };
        let settings_str = toml::to_string(&settings)?;
        fs::write(config_path, settings_str)?;
        Ok(settings)
    }

}
