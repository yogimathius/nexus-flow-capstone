use crate::error::{AppError, AppResult};
use crate::services::ai::traits::AiProvider;
use crate::services::ai::types::{Message, ModelConfig};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tracing::{error, info};

#[derive(Serialize)]
struct GithubCompletionRequest {
    messages: Vec<Message>,
    max_tokens: u32,
    model: String,
    temperature: f32,
    top_p: f32,
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
            .unwrap();

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
        };

        info!("Request body: {:?}", serde_json::to_string(&request_body));

        let response = match self
            .client
            .post(&self.url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .header("User-Agent", "Nexus-Flow/1.0")
            .json(&request_body)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                error!("Failed to send request to GitHub API: {}", e);
                return Err(AppError::HttpError(e));
            }
        };

        let status = response.status();
        info!("GitHub API response status: {}", status);

        if status.as_u16() == 401 {
            error!("Authentication failed with GitHub API - check your token");
            return Err(AppError::ApiKeyError("Invalid GitHub token".into()));
        }

        if status.as_u16() == 429 {
            error!("Rate limit exceeded with GitHub API");
            return Err(AppError::RateLimitedError);
        }

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("GitHub API error response: {}", error_text);

            return Err(AppError::ExternalServiceError(format!(
                "GitHub API returned error status: {} - {}",
                status, error_text
            )));
        }

        let text = match response.text().await {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to read response body from GitHub API: {}", e);
                return Err(AppError::HttpError(e));
            }
        };

        let completion: GithubCompletionResponse = match serde_json::from_str(&text) {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to parse response from GitHub API: {}", e);
                error!("Response was: {}", text);
                return Err(AppError::SerializationError(e));
            }
        };

        match completion.choices.first() {
            Some(choice) if !choice.message.content.is_empty() => {
                Ok(choice.message.content.clone())
            }
            Some(_) => {
                error!("Empty content returned from GitHub API");
                Err(AppError::ApplicationError("Empty content returned".into()))
            }
            None => {
                error!("No choices returned in response from GitHub API");
                Err(AppError::ApplicationError(
                    "No choices returned in response".into(),
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
