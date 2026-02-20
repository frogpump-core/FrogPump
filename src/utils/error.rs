use thiserror::Error;

pub type Result<T> = std::result::Result<T, FrogError>;

#[derive(Debug, Error)]
pub enum FrogError {
    #[error("API error: {0}")]
    Api(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Solana error: {0}")]
    Solana(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl FrogError {
    pub fn api(msg: impl Into<String>) -> Self {
        FrogError::Api(msg.into())
    }

    pub fn config(msg: impl Into<String>) -> Self {
        FrogError::Config(msg.into())
    }

    pub fn solana(msg: impl Into<String>) -> Self {
        FrogError::Solana(msg.into())
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        FrogError::Validation(msg.into())
    }

    pub fn parse(msg: impl Into<String>) -> Self {
        FrogError::Parse(msg.into())
    }
}

impl From<toml::de::Error> for FrogError {
    fn from(e: toml::de::Error) -> Self {
        FrogError::Config(format!("TOML parse error: {}", e))
    }
}

impl From<toml::ser::Error> for FrogError {
    fn from(e: toml::ser::Error) -> Self {
        FrogError::Config(format!("TOML serialize error: {}", e))
    }
}

// iteration 66
