use crate::error::AppResult;
use crate::services::mcp::types::{SymbolMeaning, TextOperation};

/// A trait defining the interface for MCP clients
/// This allows for easy mocking in tests
pub trait McpClientTrait: Send + Sync {
    fn clone_box(&self) -> Box<dyn McpClientTrait>;

    /// Process text with the specified operation
    fn process_text<'a>(
        &'a self,
        text: &'a str,
        operation: TextOperation,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<String>> + Send + 'a>>;

    /// Look up symbol meanings
    fn lookup_symbols<'a>(
        &'a self,
        symbols: Vec<&'a str>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = AppResult<Vec<SymbolMeaning>>> + Send + 'a>,
    >;

    /// List available tools from the MCP server
    fn list_tools<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Vec<String>>> + Send + 'a>>;
}

// Implement Clone for Box<dyn McpClientTrait>
impl Clone for Box<dyn McpClientTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
