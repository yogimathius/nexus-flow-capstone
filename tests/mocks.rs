use axum_server::services::mcp_client::{McpClientTrait, SymbolMeaning, TextOperation};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

/// Mock implementation of the MCP client for testing
pub struct MockMcpClient {}

impl MockMcpClient {
    pub fn new() -> Self {
        MockMcpClient {}
    }
}

impl McpClientTrait for MockMcpClient {
    fn clone_box(&self) -> Box<dyn McpClientTrait> {
        Box::new(MockMcpClient::new())
    }

    fn process_text<'a>(
        &'a self,
        text: &'a str,
        operation: TextOperation,
    ) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn Error + Send + Sync>>> + Send + 'a>>
    {
        Box::pin(async move {
            // Simple mock implementation
            match operation {
                TextOperation::Count => {
                    let char_count = text.chars().count();
                    let word_count = text.split_whitespace().count();
                    Ok(format!("Characters: {}, Words: {}", char_count, word_count))
                }
                TextOperation::Validate => Ok("Text is valid".to_string()),
                TextOperation::Format => Ok(text.to_uppercase()),
                TextOperation::Analyze => Ok("Sentiment: neutral".to_string()),
            }
        })
    }

    fn lookup_symbols<'a>(
        &'a self,
        symbols: Vec<&'a str>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<SymbolMeaning>, Box<dyn Error + Send + Sync>>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async move {
            // Return mock symbol meanings
            let mut results = Vec::new();

            for symbol in symbols {
                results.push(SymbolMeaning {
                    symbol: symbol.to_string(),
                    meaning: format!("Mock meaning for {}", symbol),
                    contexts: vec!["Testing".to_string()],
                });
            }

            Ok(results)
        })
    }
}
