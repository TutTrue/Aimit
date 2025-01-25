use home::home_dir;
use reqwest;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs;
use toml::Value;

use crate::{error::error::AimitError, models::ModelType};

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    default_model: ModelType,
    prompt: String,
    api_keys: ApiKeysConfig,
    version_needs_update: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiKeysConfig {
    gemini_api_key: Option<String>,
    deepseek_api_key: Option<String>,
}

const CONFIG_FILE: &str = ".config/aimit/Config.toml";

impl Settings {
    pub fn new() -> Result<Self, AimitError> {
        let home = home_dir().ok_or(AimitError::HomeDirectoryNotFoundError)?;
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

    pub fn save(&self) -> Result<(), AimitError> {
        let settings = toml::to_string(self)?;
        let home = home_dir().ok_or(AimitError::HomeDirectoryNotFoundError)?;
        let config_path = home.join(CONFIG_FILE);

        fs::write(config_path, settings)?;
        Ok(())
    }

    pub fn get_api_key(&self, service: ModelType) -> Result<&str, AimitError> {
        match service {
            ModelType::GEMINI => self
                .api_keys
                .gemini_api_key
                .as_deref()
                .ok_or(AimitError::ApiKeyNotFoundError),
        }
    }

    fn create_default() -> Result<Self, AimitError> {
        let home = home_dir().ok_or(AimitError::HomeDirectoryNotFoundError)?;
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
                gemini_api_key: None,
                deepseek_api_key: None,
            },
            version_needs_update: false,
        };
        let settings_str = toml::to_string(&settings)?;
        fs::write(config_path, settings_str)?;
        Ok(settings)
    }

    pub fn get_version_needs_update(&self) -> bool {
        self.version_needs_update
    }

    pub async fn version_needs_update(&mut self) -> Result<bool, AimitError> {
        let latest_version = Self::fetch_latest_version().await?;
        let current_version = Version::parse(env!("CARGO_PKG_VERSION"))?;
        self.version_needs_update = current_version < latest_version;
        self.save()?;
        Ok(current_version < latest_version)
    }

    async fn fetch_latest_version() -> Result<Version, AimitError> {
        let url = "https://raw.githubusercontent.com/MozBlbn/tuttrue-aimit/main/Cargo.toml";

        let response = reqwest::get(url).await?;

        if !response.status().is_success() {
            return Err(AimitError::FileNotFoundError(format!(
                "Failed to fetch Cargo.toml file: {}",
                response.status()
            )));
        }
        let cargo_toml_str = response.text().await?;

        let cargo_toml: Value = cargo_toml_str.parse()?;

        let version_str = cargo_toml["package"]["version"]
            .as_str()
            .ok_or(AimitError::VersionParseError)?;

        let version = Version::parse(version_str).map_err(|_| AimitError::VersionParseError)?;

        Ok(version)
    }
}
