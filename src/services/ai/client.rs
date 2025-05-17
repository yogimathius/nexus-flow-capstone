use crate::error::AppResult;
use crate::services::ai::interpreter::DreamInterpreterService;
use crate::services::ai::providers::{GitHubProvider, OpenRouterProvider};
use crate::services::ai::traits::AiClientTrait;
use crate::services::mcp::McpClientTrait;
use std::any::Any;

#[derive(Clone)]
pub struct AiClient {
    interpreter_service: DreamInterpreterService,
}

impl AiClient {
    pub fn new(openrouter_api_key: String, github_token: Option<String>) -> Self {
        let mut builder = DreamInterpreterService::builder();

        // Always add OpenRouter provider
        let openrouter_provider = OpenRouterProvider::new(openrouter_api_key);
        builder = builder.with_provider(openrouter_provider);

        // Add GitHub provider only if token is provided and it's not in test mode
        // Use a test API key to detect test mode
        if let Some(token) = github_token {
            if !token.starts_with("test_") && !token.is_empty() && token != "dummy_key" {
                let github_provider = GitHubProvider::new(token);
                builder = builder.with_provider(github_provider);
            }
        }

        let interpreter_service = builder.build();

        AiClient {
            interpreter_service,
        }
    }

    pub fn with_openrouter_url(self, url: &str) -> Self {
        let existing_providers = self.interpreter_service.providers();

        // Find the OpenRouter provider to get its API key
        let api_key = existing_providers
            .iter()
            .find_map(|provider| {
                if let Some(openrouter) = provider.as_any().downcast_ref::<OpenRouterProvider>() {
                    Some(openrouter.get_api_key().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "sk-or-test-1234567890abcdef1234567890abcdef".to_string());

        // Create a new OpenRouter provider with the URL
        let openrouter_provider = OpenRouterProvider::new(api_key).with_url(url);

        // Set up a new builder
        let mut builder = DreamInterpreterService::builder();

        // Add our new OpenRouter provider
        builder = builder.with_provider(openrouter_provider);

        // Add all other providers except OpenRouter ones
        for provider in existing_providers {
            if provider
                .as_any()
                .downcast_ref::<OpenRouterProvider>()
                .is_none()
            {
                builder = builder.with_boxed_provider(provider.clone_box());
            }
        }

        // If we had MCP client, preserve it
        if let Some(mcp_client) = self.interpreter_service.mcp_client() {
            builder = builder.with_mcp_client_ref(mcp_client.clone());
        }

        // Build the new service
        let interpreter_service = builder.build();

        AiClient {
            interpreter_service,
        }
    }

    pub fn with_mcp_client<T: McpClientTrait + 'static>(self, mcp_client: T) -> Self {
        // Get existing providers
        let existing_providers = self.interpreter_service.providers();

        // Start with a new builder
        let mut builder = DreamInterpreterService::builder();

        // Add MCP client
        builder = builder.with_mcp_client(mcp_client);

        // Add all existing providers
        for provider in existing_providers {
            builder = builder.with_boxed_provider(provider.clone_box());
        }

        // Build the new service
        let interpreter_service = builder.build();

        AiClient {
            interpreter_service,
        }
    }

    pub fn has_mcp_client(&self) -> bool {
        self.interpreter_service.has_mcp_client()
    }

    pub async fn request_dream_interpretation(&self, dream_text: &str) -> AppResult<String> {
        self.interpreter_service.interpret_dream(dream_text).await
    }
}

impl AiClientTrait for AiClient {
    fn clone_box(&self) -> Box<dyn AiClientTrait> {
        Box::new(self.clone())
    }

    fn request_dream_interpretation<'a>(
        &'a self,
        dream_text: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<String>> + Send + 'a>> {
        Box::pin(self.request_dream_interpretation(dream_text))
    }

    fn has_mcp_client(&self) -> bool {
        self.interpreter_service.has_mcp_client()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
