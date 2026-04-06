use crate::error::AppError;
use crate::models::GenerationResult;
use reqwest::Client;
use std::env;

pub enum LlmProvider {
    Mistral,
    Ollama,
}

pub struct LlmService {
    provider: LlmProvider,
    client: Client,
    endpoint: String,
    api_key: Option<String>,
    model: String,
}

impl LlmService {
    pub fn new(provider: LlmProvider, model: &str) -> Result<Self, AppError> {
        let client = Client::new();

        match provider {
            LlmProvider::Mistral => {
                let endpoint = env::var("MISTRAL_API_ENDPOINT")
                    .unwrap_or_else(|_| "https://api.mistral.ai/v1/chat/completions".to_string());
                let api_key = env::var("MISTRAL_API_KEY").ok();

                if api_key.is_none() {
                    return Err(AppError::EnvVarNotSet("MISTRAL_API_KEY".to_string()));
                }

                Ok(Self {
                    provider,
                    client,
                    endpoint,
                    api_key,
                    model: model.to_string(),
                })
            }
            LlmProvider::Ollama => {
                let endpoint = env::var("OLLAMA_ENDPOINT")
                    .unwrap_or_else(|_| "http://localhost:11434/api/generate".to_string());

                Ok(Self {
                    provider,
                    client,
                    endpoint,
                    api_key: None,
                    model: model.to_string(),
                })
            }
        }
    }

    pub fn new_mock() -> Self {
        Self {
            provider: LlmProvider::Ollama,
            client: Client::new(),
            endpoint: "mock".to_string(),
            api_key: None,
            model: "mock-model".to_string(),
        }
    }

    pub async fn generate_text(&self, prompt: &str) -> Result<String, AppError> {
        match self.provider {
            LlmProvider::Mistral => self.generate_mistral(prompt).await,
            LlmProvider::Ollama => self.generate_ollama(prompt).await,
        }
    }

    pub async fn generate_text_mock(&self, prompt: &str) -> Result<String, AppError> {
        Ok(format!("Mock response to: {}", prompt))
    }

    async fn generate_mistral(&self, prompt: &str) -> Result<String, AppError> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| AppError::EnvVarNotSet("MISTRAL_API_KEY".to_string()))?;

        let request_body = serde_json::json!({
            "model": "mistral-tiny",
            "messages": [
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.7,
        });

        let response = self
            .client
            .post(&self.endpoint)
            .bearer_auth(api_key)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::Llm(format!("API error: {}", response.status())));
        }

        let json: serde_json::Value = response.json().await?;
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AppError::Llm("Invalid API response format".to_string()))?;

        Ok(content.to_string())
    }

    async fn generate_ollama(&self, prompt: &str) -> Result<String, AppError> {
        let request_body = serde_json::json!({
            "model": &self.model,
            "prompt": prompt,
            "stream": false,
        });

        let response = self
            .client
            .post(&self.endpoint)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::Llm(format!("API error: {}", response.status())));
        }

        let json: serde_json::Value = response.json().await?;
        let content = json["response"]
            .as_str()
            .ok_or_else(|| AppError::Llm("Invalid API response format".to_string()))?;

        Ok(content.to_string())
    }

    pub async fn generate_structured(&self, prompt: &str) -> Result<GenerationResult, AppError> {
        let text = self.generate_text(prompt).await?;

        // Try to parse as JSON first
        if let Ok(result) = serde_json::from_str(&text) {
            return Ok(result);
        }

        // If not JSON, return as simple message
        Ok(GenerationResult {
            message: text,
            insight: None,
            profile_snippet: None,
        })
    }

    pub async fn generate_structured_mock(
        &self,
        prompt: &str,
    ) -> Result<GenerationResult, AppError> {
        Ok(GenerationResult {
            message: format!("Mock structured response to: {}", prompt),
            insight: Some("Mock insight".to_string()),
            profile_snippet: Some("Mock profile snippet".to_string()),
        })
    }
}
