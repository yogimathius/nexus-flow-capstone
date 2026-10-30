use crate::error::{AppError, AppResult};
use crate::services::ai::providers::{GitHubProvider, OpenRouterProvider};
use crate::services::ai::traits::AiProvider;
use crate::services::mcp::McpClientTrait;
use crate::services::mcp::types::SymbolMeaning;
use std::any::Any;
use std::sync::Arc;
use tracing::{error, warn};

use super::AiClientTrait;

#[derive(Clone)]
pub struct AiClient {
    openrouter_provider: Option<OpenRouterProvider>,
    github_provider: Option<GitHubProvider>,
    mcp_client: Option<Arc<dyn McpClientTrait>>,
}

impl AiClient {
    pub fn new(github_token: Option<String>) -> Self {
        let github_provider = github_token
            .filter(|token| !token.is_empty())
            .map(GitHubProvider::new);

        AiClient {
            openrouter_provider: None,
            github_provider,
            mcp_client: None,
        }
    }

    pub fn new_with_openrouter(openrouter_api_key: String, github_token: Option<String>) -> Self {
        let openrouter_provider = if !openrouter_api_key.is_empty()
            && openrouter_api_key != "your-openrouter-api-key-here"
        {
            Some(OpenRouterProvider::new(openrouter_api_key))
        } else {
            None
        };

        let github_provider = github_token
            .filter(|token| !token.is_empty())
            .map(GitHubProvider::new);

        AiClient {
            openrouter_provider,
            github_provider,
            mcp_client: None,
        }
    }

    pub fn with_mcp_client<T: McpClientTrait + 'static>(mut self, mcp_client: T) -> Self {
        self.mcp_client = Some(Arc::new(mcp_client));
        self
    }

    pub fn with_github_provider(mut self, provider: GitHubProvider) -> Self {
        self.github_provider = Some(provider);
        self
    }

    pub fn has_mcp_client(&self) -> bool {
        self.mcp_client.is_some()
    }

    pub async fn request_dream_interpretation(&self, dream_text: &str) -> AppResult<String> {
        // Try to get the provider (prefer OpenRouter, fallback to GitHub)
        let provider = self.get_provider().map_err(|_| {
            AppError::ServiceUnavailableError("No AI provider configured for dream interpretation".to_string())
        })?;

        let symbols = self.extract_symbols(dream_text);
        let symbol_meanings = self.lookup_symbol_meanings(&symbols).await;
        let prompt = self.build_interpretation_prompt(dream_text, symbol_meanings);

        // Try to get interpretation from the provider
        match provider.generate_completion(&prompt).await {
            Ok(interpretation) => Ok(interpretation),
            Err(e) => {
                warn!("AI provider error: {}", e);
                Err(e)
            }
        }
    }

    fn get_provider(&self) -> AppResult<&dyn AiProvider> {
        // Prefer OpenRouter provider if available
        if let Some(provider) = &self.openrouter_provider {
            return Ok(provider as &dyn AiProvider);
        }

        // Fallback to GitHub provider
        match &self.github_provider {
            Some(provider) => {
                // For real API calls, make sure the token is valid
                let token = provider.get_token();
                if token.is_empty() {
                    error!("GitHub token is empty");
                    return Err(AppError::ApplicationError(
                        "GitHub token is not configured".into(),
                    ));
                }

                Ok(provider as &dyn AiProvider)
            }
            None => {
                error!("No AI provider configured for dream interpretation");
                Err(AppError::ApplicationError(
                    "No AI provider configured. Ensure OpenRouter API key or GitHub token is properly configured.".into(),
                ))
            }
        }
    }

    async fn lookup_symbol_meanings(&self, symbols: &[String]) -> Option<Vec<SymbolMeaning>> {
        if let Some(mcp_client) = &self.mcp_client {
            match mcp_client
                .lookup_symbols(symbols.iter().map(|s| s.as_str()).collect())
                .await
            {
                Ok(meanings) => Some(meanings),
                Err(e) => {
                    error!(
                        "Could not look up symbols with MCP, continuing without symbol data: {:?}",
                        e
                    );
                    None
                }
            }
        } else {
            None
        }
    }

    fn build_interpretation_prompt(
        &self,
        dream_text: &str,
        symbol_meanings: Option<Vec<SymbolMeaning>>,
    ) -> String {
        let mut prompt = format!(
            "Offer a tentative reading of this dream for personal reflection. Avoid diagnosis, fixed meanings, and claims to know the dreamer's inner life. Distinguish the dreamer's own details from general symbolic associations.\n\nDream:\n\"{}\".",
            dream_text
        );

        if let Some(meanings) = symbol_meanings {
            if !meanings.is_empty() {
                prompt.push_str(
                    "\n\nThe symbol library offers possible associations, which may or may not fit this person's experience:\n",
                );

                for meaning in meanings {
                    prompt.push_str(&format!(
                        "- {} ({}): {}\n",
                        meaning.symbol,
                        meaning.contexts.join(", "),
                        meaning.meaning
                    ));
                }
            }
        }

        prompt.push_str("\nExplore possible themes and emotions, explain uncertainty, and invite the dreamer to consider their own associations. Keep the interpretation grounded in the dream text.");

        prompt
    }

    fn extract_symbols(&self, dream_text: &str) -> Vec<String> {
        let text = dream_text.to_lowercase();
        let common_symbols = [
            "water", "fire", "flying", "falling", "teeth", "death", "snake", "chase", "house",
            "animal", "school", "baby", "car", "moon", "sun", "star", "ocean", "mountain", "door",
            "key", "bridge", "path", "shadow", "light", "mother", "father", "child", "god",
            "devil", "money", "exam", "naked", "wedding", "funeral", "birthday",
        ];

        common_symbols
            .iter()
            .filter(|&symbol| text.contains(symbol))
            .map(|&symbol| symbol.to_string())
            .collect()
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
        self.mcp_client.is_some()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
