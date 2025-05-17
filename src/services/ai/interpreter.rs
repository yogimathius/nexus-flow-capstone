use crate::error::AppResult;
use crate::services::ai::symbols::SymbolExtractor;
use crate::services::ai::traits::AiProvider;
use crate::services::mcp::traits::McpClientTrait;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Clone)]
pub struct DreamInterpreterService {
    providers: Vec<Box<dyn AiProvider>>,
    symbol_extractor: SymbolExtractor,
    mcp_client: Option<Arc<dyn McpClientTrait>>,
}

impl DreamInterpreterService {
    pub fn builder() -> DreamInterpreterServiceBuilder {
        DreamInterpreterServiceBuilder::new()
    }

    pub fn has_mcp_client(&self) -> bool {
        self.mcp_client.is_some()
    }

    pub async fn interpret_dream(&self, dream_text: &str) -> AppResult<String> {
        info!(
            "Preparing dream interpretation request for: {:?}",
            dream_text
        );

        if self.providers.is_empty() {
            error!("No AI providers configured for dream interpretation");
            return Err(crate::error::AppError::ApplicationError(
                "No AI providers configured".into(),
            ));
        }

        let symbols = self.symbol_extractor.extract_symbols(dream_text);
        info!("Extracted symbols from dream: {:?}", symbols);

        let symbol_meanings = if let Some(mcp_client) = &self.mcp_client {
            info!("Looking up symbol meanings through MCP client");
            match mcp_client
                .lookup_symbols(symbols.iter().map(|s| s.as_str()).collect())
                .await
            {
                Ok(meanings) => {
                    info!("Successfully looked up {} symbol meanings", meanings.len());
                    Some(meanings)
                }
                Err(e) => {
                    error!("Failed to look up symbols: {:?}", e);
                    None
                }
            }
        } else {
            info!("No MCP client available for symbol lookup");
            None
        };

        let mut prompt = format!(
            "You are a skilled dream interpreter with expertise in Jungian psychology, symbolism, and dream analysis. 
            
            Please interpret this dream in a thoughtful, insightful way:
            
            \"{}\".",
            dream_text
        );

        if let Some(meanings) = symbol_meanings {
            if !meanings.is_empty() {
                prompt.push_str(
                    "\n\nHere are known symbolic meanings that may be relevant to this dream:\n",
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

        prompt.push_str("\nProvide analysis of the key symbols, potential meanings, and psychological insights from this dream. Consider archetypes, personal associations, and emotional resonance.");

        info!(
            "Created dream interpretation prompt, trying {} providers",
            self.providers.len()
        );

        let mut last_error = None;
        for (i, provider) in self.providers.iter().enumerate() {
            info!("Trying provider #{} for dream interpretation", i + 1);
            match provider.generate_completion(&prompt).await {
                Ok(interpretation) => {
                    info!("Successfully got interpretation from provider #{}", i + 1);
                    return Ok(interpretation);
                }
                Err(e) => {
                    error!("Provider #{} failed: {:?}", i + 1, e);
                    last_error = Some(e);
                }
            }
        }

        if let Some(e) = last_error {
            error!("All providers failed, last error was: {:?}", e);
            return Err(e);
        }

        error!("All AI providers failed but no specific error was captured");
        Err(crate::error::AppError::ExternalServiceError(
            "All AI providers failed to generate an interpretation".into(),
        ))
    }

    pub fn providers(&self) -> &Vec<Box<dyn AiProvider>> {
        &self.providers
    }

    pub fn mcp_client(&self) -> Option<&Arc<dyn McpClientTrait>> {
        self.mcp_client.as_ref()
    }
}

pub struct DreamInterpreterServiceBuilder {
    providers: Vec<Box<dyn AiProvider>>,
    symbol_extractor: Option<SymbolExtractor>,
    mcp_client: Option<Arc<dyn McpClientTrait>>,
}

impl DreamInterpreterServiceBuilder {
    pub fn new() -> Self {
        DreamInterpreterServiceBuilder {
            providers: Vec::new(),
            symbol_extractor: None,
            mcp_client: None,
        }
    }

    pub fn with_provider(mut self, provider: impl AiProvider + 'static) -> Self {
        self.providers.push(Box::new(provider));
        self
    }

    pub fn with_boxed_provider(mut self, provider: Box<dyn AiProvider>) -> Self {
        self.providers.push(provider);
        self
    }

    pub fn with_symbol_extractor(mut self, extractor: SymbolExtractor) -> Self {
        self.symbol_extractor = Some(extractor);
        self
    }

    pub fn with_mcp_client<T: McpClientTrait + 'static>(mut self, client: T) -> Self {
        self.mcp_client = Some(Arc::new(client));
        self
    }

    pub fn with_mcp_client_ref(mut self, client: Arc<dyn McpClientTrait>) -> Self {
        self.mcp_client = Some(client);
        self
    }

    pub fn build(self) -> DreamInterpreterService {
        DreamInterpreterService {
            providers: self.providers,
            symbol_extractor: self.symbol_extractor.unwrap_or_else(SymbolExtractor::new),
            mcp_client: self.mcp_client,
        }
    }
}
