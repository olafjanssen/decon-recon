use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
    
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("LLM error: {0}")]
    Llm(String),
    
    #[error("Campaign not found: {0}")]
    CampaignNotFound(String),
    
    #[error("Character not found: {0}")]
    CharacterNotFound(String),
    
    #[error("Submodality not found: {0}")]
    SubmodalityNotFound(String),
    
    #[error("Invalid action: {0}")]
    InvalidAction(String),
    
    #[error("Environment variable not set: {0}")]
    EnvVarNotSet(String),
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Llm(err.to_string())
    }
}
