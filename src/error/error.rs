#[derive(Debug)]
pub enum AimitError {
    RepoNotFound,
    NoDiffFound,
    GitError,
    ApiKeyNotFoundError,
    HomeDirectoryNotFoundError,
    IoError(std::io::Error),
    TomlDeserializationError(toml::de::Error),
    TomlSerError(toml::ser::Error),
    ReqwestError(reqwest::Error),
    FileNotFoundError(String),
    VersionParseError,
    SemverError(semver::Error),
}

impl std::fmt::Display for AimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AimitError::RepoNotFound => write!(f, "No repository found in the current directory"),
            AimitError::NoDiffFound => write!(f, "No staged changes found"),
            AimitError::GitError => write!(f, "Git error"),
            AimitError::ApiKeyNotFoundError => {
                write!(f, "API key not found please run `aimit -k <your_api_key>`")
            }
            AimitError::HomeDirectoryNotFoundError => write!(f, "Home directory not found"),
            AimitError::IoError(err) => write!(f, "IO error: {}", err),
            AimitError::TomlDeserializationError(err) => {
                write!(f, "Toml deserialization error: {}", err)
            }
            AimitError::TomlSerError(err) => write!(f, "Toml serialization error: {}", err),
            AimitError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
            AimitError::FileNotFoundError(path) => write!(f, "File not found: {}", path),
            AimitError::VersionParseError => write!(f, "Error parsing version"),
            AimitError::SemverError(err) => write!(f, "Semver error: {}", err),
        }
    }
}

impl From<std::io::Error> for AimitError {
    fn from(err: std::io::Error) -> Self {
        AimitError::IoError(err)
    }
}

impl From<toml::de::Error> for AimitError {
    fn from(err: toml::de::Error) -> AimitError {
        AimitError::TomlDeserializationError(err)
    }
}

impl From<toml::ser::Error> for AimitError {
    fn from(error: toml::ser::Error) -> Self {
        AimitError::TomlSerError(error)
    }
}

impl From<reqwest::Error> for AimitError {
    fn from(error: reqwest::Error) -> Self {
        AimitError::ReqwestError(error)
    }
}

impl From<semver::Error> for AimitError {
    fn from(err: semver::Error) -> AimitError {
        AimitError::SemverError(err)
    }
}
