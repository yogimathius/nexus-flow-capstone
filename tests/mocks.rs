use axum_server::AppResult;
use axum_server::services::mcp::{McpClientTrait, SymbolMeaning, TextOperation};
use std::future::Future;
use std::pin::Pin;

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
    ) -> Pin<Box<dyn Future<Output = AppResult<String>> + Send + 'a>> {
        Box::pin(async move {
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
    ) -> Pin<Box<dyn Future<Output = AppResult<Vec<SymbolMeaning>>> + Send + 'a>> {
        Box::pin(async move {
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

    fn list_tools<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = AppResult<Vec<String>>> + Send + 'a>> {
        Box::pin(async move {
            Ok(vec![
                "get_symbols".to_string(),
                "search_symbols".to_string(),
                "get_categories".to_string(),
            ])
        })
    }
}
