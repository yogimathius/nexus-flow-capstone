use crate::error::{AppError, AppResult};
use crate::services::mcp::operations::try_mcp_processing;
use crate::services::mcp::symbols::SymbolLookup;
use crate::services::mcp::traits::McpClientTrait;
use crate::services::mcp::types::{SymbolMeaning, TextOperation};
use rmcp::ServiceExt;
use tracing::{error, info};

pub struct McpClient {
    client: rmcp::service::RunningService<rmcp::RoleClient, rmcp::model::InitializeRequestParam>,
}

impl McpClientTrait for McpClient {
    fn clone_box(&self) -> Box<dyn McpClientTrait> {
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
    pub async fn new() -> AppResult<Self> {
        info!("Initializing MCP client...");

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

    async fn try_connect_to_external_server() -> AppResult<
        rmcp::service::RunningService<rmcp::RoleClient, rmcp::model::InitializeRequestParam>,
    > {
        use rmcp::{
            model::{ClientCapabilities, ClientInfo, Implementation},
            transport::SseTransport,
        };

        let url = std::env::var("MCP_SERVER_URL")
            .unwrap_or_else(|_| "http://localhost:3002/sse".to_string());

        info!("Attempting to connect to external MCP server at {}", url);

        match SseTransport::start(&url).await {
            Ok(transport) => {
                info!("Connected to external MCP server at {}", url);

                let client_info = ClientInfo {
                    protocol_version: Default::default(),
                    capabilities: ClientCapabilities::default(),
                    client_info: Implementation {
                        name: "Dream Interpreter MCP Client".to_string(),
                        version: "0.1.0".to_string(),
                    },
                };

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

    async fn process_text_impl(&self, text: &str, operation: TextOperation) -> AppResult<String> {
        info!("Processing text with operation: {:?}", operation);

        match try_mcp_processing(&self.client, text, &operation).await {
            Ok(result) => {
                info!("Successfully processed text through MCP");
                Ok(result)
            }
            Err(e) => {
                info!("MCP processing unavailable: {}", e);
                Err(AppError::McpError(format!(
                    "MCP text processing unavailable: {}",
                    e
                )))
            }
        }
    }

    async fn lookup_symbols_impl(&self, symbols: Vec<&str>) -> AppResult<Vec<SymbolMeaning>> {
        info!("Looking up {} symbols from ontology", symbols.len());

        match SymbolLookup::try_mcp_lookup(&self.client, &symbols).await {
            Ok(result) => {
                info!("Successfully looked up symbols through MCP");
                Ok(result)
            }
            Err(e) => {
                info!("MCP symbol lookup unavailable: {}", e);
                Err(AppError::McpError(format!(
                    "MCP symbol lookup unavailable: {}",
                    e
                )))
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
    #[ignore = "Requires a running MCP server"]
    async fn test_mcp_client() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info,rmcp=debug")
            .try_init();

        info!("Starting MCP client test");

        match McpClient::new().await {
            Ok(client) => {
                info!("Successfully created MCP client");

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
                        info!("Text processing failed as expected: {:?}", e);
                        // We expect this to fail without local fallback, so don't panic
                        match e {
                            AppError::McpError(message) => {
                                // Verify the error message contains the expected text
                                assert!(
                                    message.contains("MCP text processing unavailable"),
                                    "Error should indicate MCP processing unavailable"
                                );
                            }
                            _ => panic!("Unexpected error type: {:?}", e),
                        }
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
