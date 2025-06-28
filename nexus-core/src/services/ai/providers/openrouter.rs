use crate::error::AppResult;
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
struct CompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct CompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Clone)]
pub struct OpenRouterProvider {
    api_key: String,
    client: reqwest::Client,
    url: String,
    model_config: ModelConfig,
}

impl OpenRouterProvider {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        OpenRouterProvider {
            api_key,
            client,
            url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
            model_config: ModelConfig {
                name: "deepseek/deepseek-chat-v3-0324:free".to_string(),
                temperature: 0.7,
                top_p: 0.9,
                max_tokens: None,
            },
        }
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn with_model_config(mut self, config: ModelConfig) -> Self {
        self.model_config = config;
        self
    }

    async fn request_completion(&self, prompt: &str) -> AppResult<String> {
        info!(
            "Sending request to OpenRouter API with model: {}",
            self.model_config.name
        );

        let request_body = CompletionRequest {
            model: self.model_config.name.clone(),
            messages: vec![Message::user(prompt)],
        };

        let response = match self
            .client
            .post(&self.url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://localhost:3000") // Required by OpenRouter
            .header("X-Title", "Dream Interpreter") // Required by OpenRouter
            .json(&request_body)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                error!("Failed to send request to OpenRouter API: {}", e);
                return Err(crate::error::AppError::HttpError(e));
            }
        };

        let status = response.status();
        info!("Received response from OpenRouter with status: {}", status);

        if status.as_u16() == 401 {
            error!("Authentication failed with OpenRouter API - check your API key");
            return Err(crate::error::AppError::ApiKeyError(
                "Invalid OpenRouter API key".into(),
            ));
        }

        if status.as_u16() == 429 {
            error!("Rate limit exceeded with OpenRouter API");
            return Err(crate::error::AppError::RateLimitedError);
        }

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("OpenRouter API error response: {}", error_text);

            return Err(crate::error::AppError::ExternalServiceError(format!(
                "OpenRouter API returned error status: {} - {}",
                status, error_text
            )));
        }

        let response_text = match response.text().await {
            Ok(text) => text,
            Err(e) => {
                error!("Failed to read response body from OpenRouter API: {}", e);
                return Err(crate::error::AppError::HttpError(e));
            }
        };

        let completion: CompletionResponse = match serde_json::from_str(&response_text) {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to parse response from OpenRouter API: {}", e);
                error!("Response was: {}", response_text);
                return Err(crate::error::AppError::SerializationError(e));
            }
        };

        match completion.choices.first() {
            Some(choice) => {
                let content = choice.message.content.clone();
                if content.is_empty() {
                    warn!("Warning: Received empty content from OpenRouter API");
                    return Ok("The AI model couldn't process your request at this time. Please try again later.".to_string());
                }
                Ok(content)
            }
            None => {
                error!("No choices returned in response from OpenRouter API");
                Err(crate::error::AppError::ApplicationError(
                    "No choices returned in response".into(),
                ))
            }
        }
    }
}

impl AiProvider for OpenRouterProvider {
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
