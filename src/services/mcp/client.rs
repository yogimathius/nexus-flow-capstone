use crate::error::{AppError, AppResult};
use crate::services::mcp::operations::TextProcessor;
use crate::services::mcp::symbols::SymbolLookup;
use crate::services::mcp::traits::McpClientTrait;
use crate::services::mcp::types::{SymbolMeaning, TextOperation};
use rmcp::ServiceExt;
use tracing::{error, info};

/// Main MCP client implementation
pub struct McpClient {
    client: rmcp::service::RunningService<rmcp::RoleClient, rmcp::model::InitializeRequestParam>,
}

// Implement the trait for the real client
impl McpClientTrait for McpClient {
    fn clone_box(&self) -> Box<dyn McpClientTrait> {
        // This is a stub that will panic if actually used
        // In a real application, use Arc/Mutex instead
        panic!("McpClient cannot be cloned directly; use Arc<dyn McpClientTrait> instead")
    }

    fn process_text<'a>(
        &'a self,
        text: &'a str,
        operation: TextOperation,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<String>> + Send + 'a>> {
        Box::pin(self.process_text_impl(text, operation))
    }

    fn lookup_symbols<'a>(
        &'a self,
        symbols: Vec<&'a str>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = AppResult<Vec<SymbolMeaning>>> + Send + 'a>,
    > {
        Box::pin(self.lookup_symbols_impl(symbols))
    }

    fn list_tools<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Vec<String>>> + Send + 'a>>
    {
        Box::pin(self.list_tools_impl())
    }
}

impl McpClient {
    /// Create a new MCP client
    pub async fn new() -> AppResult<Self> {
        info!("Initializing MCP client...");

        // Try to connect to an external MCP server
        match Self::try_connect_to_external_server().await {
            Ok(client) => {
                info!("Successfully connected to external MCP server");
                Ok(Self { client })
            }
            Err(e) => {
                info!("Could not connect to external MCP server: {}", e);
                Err(AppError::McpError(format!(
                    "MCP server connection failed: {}",
                    e
                )))
            }
        }
    }

    /// Attempt to connect to an external MCP server
    async fn try_connect_to_external_server() -> AppResult<
        rmcp::service::RunningService<rmcp::RoleClient, rmcp::model::InitializeRequestParam>,
    > {
        use rmcp::{
            model::{ClientCapabilities, ClientInfo, Implementation},
            transport::SseTransport,
        };

        // First, try connecting to a local MCP server (the symbol-ontology service)
        let url = std::env::var("MCP_SERVER_URL")
            .unwrap_or_else(|_| "http://localhost:3002/sse".to_string());

        info!("Attempting to connect to external MCP server at {}", url);

        match SseTransport::start(&url).await {
            Ok(transport) => {
                info!("Connected to external MCP server at {}", url);

                // Create client info
                let client_info = ClientInfo {
                    protocol_version: Default::default(),
                    capabilities: ClientCapabilities::default(),
                    client_info: Implementation {
                        name: "Dream Interpreter MCP Client".to_string(),
                        version: "0.1.0".to_string(),
                    },
                };

                // Initialize the MCP client
                match client_info.serve(transport).await {
                    Ok(client) => {
                        info!("Successfully initialized MCP client with external server");
                        Ok(client)
                    }
                    Err(e) => {
                        error!(
                            "Failed to initialize MCP client with external server: {:?}",
                            e
                        );
                        Err(AppError::McpError(format!(
                            "Failed to initialize MCP client with external server: {}",
                            e
                        )))
                    }
                }
            }
            Err(e) => {
                info!(
                    "Could not connect to external MCP server at {}: {:?}",
                    url, e
                );
                Err(AppError::McpError(format!(
                    "Could not connect to external MCP server: {}",
                    e
                )))
            }
        }
    }

    // Implementation methods used by the trait methods
    async fn process_text_impl(&self, text: &str, operation: TextOperation) -> AppResult<String> {
        info!("Processing text with operation: {:?}", operation);

        // First attempt to use MCP if available
        match TextProcessor::try_mcp_processing(&self.client, text, &operation).await {
            Ok(result) => {
                info!("Successfully processed text through MCP");
                return Ok(result);
            }
            Err(e) => {
                info!("MCP processing unavailable: {}. Using local processing.", e);
                // Fall back to local processing
                TextProcessor::process_locally(text, operation)
            }
        }
    }

    async fn lookup_symbols_impl(&self, symbols: Vec<&str>) -> AppResult<Vec<SymbolMeaning>> {
        info!("Looking up {} symbols from ontology", symbols.len());

        // First attempt to use MCP if available
        match SymbolLookup::try_mcp_lookup(&self.client, &symbols).await {
            Ok(result) => {
                info!("Successfully looked up symbols through MCP");
                return Ok(result);
            }
            Err(e) => {
                info!(
                    "MCP symbol lookup unavailable: {}. Using local ontology.",
                    e
                );
                // Fall back to local lookup
                SymbolLookup::lookup_locally(symbols)
            }
        }
    }

    async fn list_tools_impl(&self) -> AppResult<Vec<String>> {
        info!("Listing available MCP tools");

        match self.client.list_tools(Default::default()).await {
            Ok(tools) => {
                let tool_names: Vec<String> =
                    tools.tools.iter().map(|t| t.name.to_string()).collect();

                info!("Available MCP tools: {:?}", tool_names);
                Ok(tool_names)
            }
            Err(e) => {
                error!("Failed to list MCP tools: {:?}", e);
                Err(AppError::McpError(format!(
                    "Failed to list MCP tools: {}",
                    e
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_client() {
        // Initialize logging for tests
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info,rmcp=debug")
            .try_init();

        // This test initializes the MCP client
        info!("Starting MCP client test");

        // Try to create the client
        match McpClient::new().await {
            Ok(client) => {
                info!("Successfully created MCP client");

                // Try some local text processing
                let sample_text = "this is a sample text. it needs formatting!";
                match client
                    .process_text(sample_text, TextOperation::Format)
                    .await
                {
                    Ok(result) => {
                        info!("Successfully processed text:");
                        info!("Input: {}", sample_text);
                        info!("Output: {}", result);
                        assert!(!result.is_empty(), "Result should not be empty");
                    }
                    Err(e) => {
                        error!("Failed to process text: {:?}", e);
                        panic!("Text processing failed");
                    }
                }
            }
            Err(e) => {
                error!("Failed to create MCP client: {:?}", e);
                panic!("MCP client creation failed");
            }
        }
    }
}
