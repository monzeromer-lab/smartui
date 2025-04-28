use thiserror::Error;

pub type Result<T> = std::result::Result<T, GmError>;

#[derive(Error, Debug)]
pub enum GmError {
    #[error("Gemini API error: {0}")]
    GeminiApi(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("API key not found. Please set GEMINI_API_KEY environment variable")]
    ApiKeyNotFound,

    #[error("Invalid response from Gemini API: {0}")]
    InvalidResponse(String),
} 