use crate::error::{AppError, AppResult};
use crate::services::ai::traits::AiProvider;
use crate::services::ai::types::{Message, ModelConfig};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tracing::{error, info, warn};

#[derive(Serialize)]
struct GithubCompletionRequest {
    messages: Vec<Message>,
    max_tokens: u32,
    model: String,
    temperature: f32,
    top_p: f32,
    publisher: String,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct GithubChoice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct GithubCompletionResponse {
    choices: Vec<GithubChoice>,
}

#[derive(Clone)]
pub struct GitHubProvider {
    token: String,
    client: reqwest::Client,
    model_config: ModelConfig,
    url: String,
}

impl GitHubProvider {
    pub fn new(token: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap_or_else(|e| {
                warn!("Failed to build HTTP client with custom timeout: {}", e);
                reqwest::Client::new()
            });

        GitHubProvider {
            token,
            client,
            model_config: ModelConfig {
                name: "openai/gpt-4.1".to_string(),
                temperature: 0.7,
                top_p: 0.9,
                max_tokens: Some(800),
            },
            url: "https://models.github.ai/inference/chat/completions".to_string(),
        }
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn with_model_config(mut self, config: ModelConfig) -> Self {
        self.model_config = config;
        self
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    async fn request_completion(&self, prompt: &str) -> AppResult<String> {
        info!(
            "Using GitHub API for completion with model: {}",
            self.model_config.name
        );

        let system_message = Message {
            role: "system".to_string(),
            content: "You are a dream interpreter with expertise in psychology, symbolism, and mythology. Analyze the provided dream and explain its potential meanings based on common psychological interpretations.".to_string(),
        };

        let request_body = GithubCompletionRequest {
            messages: vec![system_message, Message::user(prompt)],
            max_tokens: self.model_config.max_tokens.unwrap_or(800),
            model: self.model_config.name.clone(),
            temperature: self.model_config.temperature,
            top_p: self.model_config.top_p,
            publisher: "openai".to_string(),
        };

        let response = self
            .client
            .post(&self.url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .header("User-Agent", "Nexus-Flow/1.0")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                error!("GitHub API request failed: {}", e);
                if e.is_timeout() {
                    AppError::ExternalServiceError(format!("GitHub API request timed out: {}", e))
                } else if e.is_connect() {
                    AppError::ExternalServiceError(format!("Connection error to GitHub API: {}", e))
                } else {
                    AppError::HttpError(e)
                }
            })?;

        let status = response.status();
        info!("GitHub API response status: {}", status);

        match status.as_u16() {
            401 => {
                error!("Authentication failed with GitHub API - invalid token");
                return Err(AppError::ApiKeyError("Invalid GitHub API token or insufficient permissions".into()));
            }
            403 => {
                error!("GitHub API request forbidden - check permissions");
                return Err(AppError::AuthorizationError);
            }
            404 => {
                error!("GitHub API endpoint not found");
                return Err(AppError::ExternalServiceError("GitHub API endpoint not found".into()));
            }
            429 => {
                error!("Rate limit exceeded with GitHub API");
                return Err(AppError::RateLimitedError);
            }
            500..=599 => {
                error!("GitHub API server error");
                let error_text = response.text().await.unwrap_or_default();
                return Err(AppError::ExternalServiceError(format!(
                    "GitHub API server error ({}): {}",
                    status, error_text
                )));
            }
            _ if !status.is_success() => {
                let error_text = response.text().await.unwrap_or_default();
                error!("GitHub API error response: {}", error_text);
                return Err(AppError::ExternalServiceError(format!(
                    "GitHub API returned error status: {} - {}",
                    status, error_text
                )));
            }
            _ => {}
        }

        let text = response.text().await.map_err(|e| {
            error!("Failed to read response body from GitHub API: {}", e);
            AppError::HttpError(e)
        })?;

        let completion: GithubCompletionResponse = serde_json::from_str(&text).map_err(|e| {
            error!("Failed to parse response from GitHub API: {}", e);
            error!("Response was: {}", text);
            AppError::SerializationError(e)
        })?;

        match completion.choices.first() {
            Some(choice) if !choice.message.content.is_empty() => {
                Ok(choice.message.content.clone())
            }
            Some(_) => {
                error!("Empty content returned from GitHub API");
                Err(AppError::ApplicationError("GitHub API returned empty response content".into()))
            }
            None => {
                error!("No choices returned in response from GitHub API");
                Err(AppError::ApplicationError(
                    "GitHub API response missing expected 'choices' data".into(),
                ))
            }
        }
    }
}

impl AiProvider for GitHubProvider {
    fn generate_completion<'a>(
        &'a self,
        prompt: &'a str,
    ) -> Pin<Box<dyn Future<Output = AppResult<String>> + Send + 'a>> {
        Box::pin(self.request_completion(prompt))
    }

    fn clone_box(&self) -> Box<dyn AiProvider> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
