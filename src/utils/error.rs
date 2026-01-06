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

