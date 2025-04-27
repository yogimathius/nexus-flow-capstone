use rmcp::{ServiceExt, object, transport::TokioChildProcess};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use tokio::process::Command;
use tracing::{debug, error, info};

/// A trait defining the interface for MCP clients
/// This allows for easy mocking in tests
pub trait McpClientTrait: Send + Sync {
    fn clone_box(&self) -> Box<dyn McpClientTrait>;

    /// Process text with the specified operation
    fn process_text<'a>(
        &'a self,
        text: &'a str,
        operation: TextOperation,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<String, Box<dyn Error + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;

    /// Look up symbol meanings
    fn lookup_symbols<'a>(
        &'a self,
        symbols: Vec<&'a str>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<Vec<SymbolMeaning>, Box<dyn Error + Send + Sync>>,
                > + Send
                + 'a,
        >,
    >;

    /// List available tools from the MCP server
    fn list_tools<'a>(
        &'a self,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Vec<String>, Box<dyn Error + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;
}

// Implement Clone for Box<dyn McpClientTrait>
impl Clone for Box<dyn McpClientTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

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
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<String, Box<dyn Error + Send + Sync>>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(self.process_text_impl(text, operation))
    }

    fn lookup_symbols<'a>(
        &'a self,
        symbols: Vec<&'a str>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<Vec<SymbolMeaning>, Box<dyn Error + Send + Sync>>,
                > + Send
                + 'a,
        >,
    > {
        Box::pin(self.lookup_symbols_impl(symbols))
    }

    fn list_tools<'a>(
        &'a self,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Vec<String>, Box<dyn Error + Send + Sync>>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(self.list_tools_impl())
    }
}

impl McpClient {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        info!("Initializing MCP client...");

        // Try to connect to an external MCP server first
        if let Ok(client) = Self::try_connect_to_external_server().await {
            info!("Successfully connected to external MCP server");
            return Ok(Self { client });
        }

        info!("No external MCP server found, starting built-in server...");

        // Initialize the MCP client with the server-everything implementation
        let mut command = Command::new("npx");
        command
            .arg("-y")
            .arg("@modelcontextprotocol/server-everything");

        debug!("Starting MCP server with command: {:?}", command);

        let transport = match TokioChildProcess::new(&mut command) {
            Ok(process) => process,
            Err(e) => {
                error!("Failed to start MCP server process: {:?}", e);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to start MCP server process: {}", e),
                )));
            }
        };

        // Create client info
        use rmcp::model::{ClientCapabilities, ClientInfo, Implementation};
        let client_info = ClientInfo {
            protocol_version: Default::default(),
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: "Dream Interpreter MCP Client".to_string(),
                version: "0.1.0".to_string(),
            },
        };

        let client = match client_info.serve(transport).await {
            Ok(client) => {
                info!("MCP client successfully initialized with built-in server");
                client
            }
            Err(e) => {
                error!("Failed to initialize MCP client: {:?}", e);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to initialize MCP client: {}", e),
                )));
            }
        };

        Ok(Self { client })
    }

    async fn try_connect_to_external_server() -> Result<
        rmcp::service::RunningService<rmcp::RoleClient, rmcp::model::InitializeRequestParam>,
        Box<dyn Error + Send + Sync>,
    > {
        use rmcp::{
            model::{ClientCapabilities, ClientInfo, Implementation},
            transport::SseTransport,
        };

        // First, try connecting to a local MCP server (the symbol-ontology service)
        let url = std::env::var("MCP_SERVER_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:3002/sse".to_string());

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
                        Err(format!(
                            "Failed to initialize MCP client with external server: {}",
                            e
                        )
                        .into())
                    }
                }
            }
            Err(e) => {
                info!(
                    "Could not connect to external MCP server at {}: {:?}",
                    url, e
                );
                Err(format!("Could not connect to external MCP server: {}", e).into())
            }
        }
    }

    // Private implementation methods used by the trait methods
    async fn process_text_impl(
        &self,
        text: &str,
        operation: TextOperation,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        info!("Processing text with operation: {:?}", operation);

        // First attempt to use MCP if available
        match self.try_mcp_processing(text, &operation).await {
            Ok(result) => {
                info!("Successfully processed text through MCP");
                return Ok(result);
            }
            Err(e) => {
                info!("MCP processing unavailable: {}. Using local processing.", e);
                // Fall back to local processing
                self.process_locally(text, operation)
            }
        }
    }

    async fn try_mcp_processing(
        &self,
        text: &str,
        operation: &TextOperation,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Try to use MCP for processing
        let op_name = match operation {
            TextOperation::Count => "text_count",
            TextOperation::Validate => "text_validate",
            TextOperation::Format => "text_format",
            TextOperation::Analyze => "text_analyze",
        };

        debug!("Attempting to use MCP tool: {}", op_name);

        match self
            .client
            .call_tool(rmcp::model::CallToolRequestParam {
                name: op_name.into(),
                arguments: Some(object!({
                    "text": text
                })),
            })
            .await
        {
            Ok(result) => {
                let json_result = serde_json::to_value(&result)?;
                if let Some(result_str) = json_result.as_str() {
                    return Ok(result_str.to_string());
                }

                if let Some(result_obj) = json_result.as_object() {
                    return Ok(serde_json::to_string_pretty(result_obj)?);
                }

                Ok(json_result.to_string())
            }
            Err(e) => {
                debug!("MCP tool unavailable: {:?}", e);
                Err(format!("MCP tool '{}' not available", op_name).into())
            }
        }
    }

    fn process_locally(
        &self,
        text: &str,
        operation: TextOperation,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        info!("Processing text locally with operation: {:?}", operation);

        match operation {
            TextOperation::Count => {
                let char_count = text.chars().count();
                let word_count = text.split_whitespace().count();
                let line_count = text.lines().count();

                Ok(format!(
                    "Text statistics:\n- Characters: {}\n- Words: {}\n- Lines: {}",
                    char_count, word_count, line_count
                ))
            }
            TextOperation::Validate => {
                // Simple validation: check if text is empty, all whitespace, or contains special patterns
                let is_empty = text.trim().is_empty();
                let has_numbers = text.chars().any(|c| c.is_numeric());
                let has_special = text
                    .chars()
                    .any(|c| !c.is_alphanumeric() && !c.is_whitespace());

                let mut validations = Vec::new();
                if is_empty {
                    validations.push("Text is empty or whitespace only");
                }
                if has_numbers {
                    validations.push("Contains numeric characters");
                }
                if has_special {
                    validations.push("Contains special characters");
                }

                let result = if validations.is_empty() {
                    "Text is valid plain text (alphabetic characters only)".to_string()
                } else {
                    format!("Validation results:\n- {}", validations.join("\n- "))
                };

                Ok(result)
            }
            TextOperation::Format => {
                // Format text: capitalize sentences, normalize whitespace
                let sentences = text
                    .split_terminator(['.', '!', '?'])
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| {
                        let s = s.trim();
                        if let Some(first_char) = s.chars().next() {
                            let first_upper = first_char.to_uppercase().collect::<String>();
                            if s.len() > 1 {
                                first_upper + &s[first_char.len_utf8()..] + "."
                            } else {
                                first_upper + "."
                            }
                        } else {
                            String::new()
                        }
                    })
                    .collect::<Vec<_>>();

                Ok(format!("Formatted text:\n{}", sentences.join(" ")))
            }
            TextOperation::Analyze => {
                // Sentiment analysis
                let text_lower = text.to_lowercase();
                let positive_words = [
                    "good",
                    "great",
                    "happy",
                    "excellent",
                    "wonderful",
                    "awesome",
                    "love",
                    "like",
                ];
                let negative_words = [
                    "bad", "awful", "sad", "terrible", "hate", "dislike", "poor", "worst",
                ];

                let pos_count = positive_words
                    .iter()
                    .filter(|w| text_lower.contains(*w))
                    .count();

                let neg_count = negative_words
                    .iter()
                    .filter(|w| text_lower.contains(*w))
                    .count();

                let sentiment = if pos_count > neg_count {
                    "positive"
                } else if neg_count > pos_count {
                    "negative"
                } else {
                    "neutral"
                };

                Ok(format!(
                    "Text analysis:\n- Sentiment: {}\n- Positive indicators: {}\n- Negative indicators: {}",
                    sentiment, pos_count, neg_count
                ))
            }
        }
    }

    async fn lookup_symbols_impl(
        &self,
        symbols: Vec<&str>,
    ) -> Result<Vec<SymbolMeaning>, Box<dyn Error + Send + Sync>> {
        info!("Looking up {} symbols from ontology", symbols.len());

        // First attempt to use MCP if available
        match self.try_mcp_symbol_lookup(&symbols).await {
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
                self.lookup_symbols_locally(symbols)
            }
        }
    }

    async fn try_mcp_symbol_lookup(
        &self,
        symbols: &[&str],
    ) -> Result<Vec<SymbolMeaning>, Box<dyn Error + Send + Sync>> {
        debug!("Attempting to look up symbols via MCP: {:?}", symbols);

        if symbols.is_empty() {
            // If no symbols provided, just return empty result
            return Ok(Vec::new());
        }

        // For each symbol, try to get information
        let mut results = Vec::new();

        for symbol in symbols {
            // Use search_symbols for precise lookup as it requires a query parameter
            let params = serde_json::json!({
                "query": symbol,
                "limit": 5 // Limit results to avoid too much data
            });

            debug!(
                "Searching for symbol '{}' using search_symbols tool",
                symbol
            );

            // Use a timeout to prevent hanging
            match tokio::time::timeout(
                std::time::Duration::from_secs(3),
                self.client.call_tool(rmcp::model::CallToolRequestParam {
                    name: "search_symbols".into(),
                    arguments: params.as_object().cloned(),
                }),
            )
            .await
            {
                Ok(Ok(result)) => {
                    debug!("Successfully found symbols matching '{}'", symbol);

                    // Parse the result JSON to extract symbol meanings
                    if let Ok(json_value) = serde_json::to_value(&result) {
                        debug!("Symbol search result: {:?}", json_value);

                        if let Some(items) = json_value.as_array() {
                            for item in items {
                                if let Some(obj) = item.as_object() {
                                    let symbol_name = obj
                                        .get("name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or(symbol)
                                        .to_string();

                                    let meaning = obj
                                        .get("description")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("No description available")
                                        .to_string();

                                    let category = obj
                                        .get("category")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("unknown");

                                    results.push(SymbolMeaning {
                                        symbol: symbol_name,
                                        meaning,
                                        contexts: vec![category.to_string()],
                                    });
                                }
                            }
                        }
                    }
                }
                Ok(Err(e)) => {
                    debug!("MCP search_symbols tool error for '{}': {:?}", symbol, e);
                    // Continue to try other symbols
                }
                Err(_) => {
                    debug!("MCP search_symbols tool timed out for '{}'", symbol);
                    // Continue to try other symbols
                }
            }
        }

        if !results.is_empty() {
            debug!("Found {} symbol meanings via MCP", results.len());
            return Ok(results);
        }

        // If no results found via specific lookup, try the general get_symbols API
        debug!("No specific symbols found, trying general symbols API");

        let params = serde_json::json!({
            "limit": 10
        });

        match tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.client.call_tool(rmcp::model::CallToolRequestParam {
                name: "get_symbols".into(),
                arguments: params.as_object().cloned(),
            }),
        )
        .await
        {
            Ok(Ok(result)) => {
                debug!("Successfully retrieved general symbols");

                if let Ok(json_value) = serde_json::to_value(&result) {
                    if let Some(items) = json_value.as_array() {
                        for item in items {
                            if let Some(obj) = item.as_object() {
                                let symbol_name = obj
                                    .get("name")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("unknown")
                                    .to_string();

                                let meaning = obj
                                    .get("description")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("No description available")
                                    .to_string();

                                let category = obj
                                    .get("category")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("general");

                                results.push(SymbolMeaning {
                                    symbol: symbol_name,
                                    meaning,
                                    contexts: vec![category.to_string()],
                                });
                            }
                        }
                    }
                }

                debug!("Found {} general symbols via MCP", results.len());
                Ok(results)
            }
            Ok(Err(e)) => {
                debug!("MCP get_symbols tool error: {:?}", e);
                Err(format!("MCP symbol lookup tool error: {}", e).into())
            }
            Err(_) => {
                debug!("MCP get_symbols tool timed out");
                Err("MCP symbol lookup timed out".into())
            }
        }
    }

    fn lookup_symbols_locally(
        &self,
        symbols: Vec<&str>,
    ) -> Result<Vec<SymbolMeaning>, Box<dyn Error + Send + Sync>> {
        info!("Looking up symbols in local ontology database");

        // Implement a basic local lookup for common symbols
        let mut results = Vec::new();

        for symbol in symbols {
            let meaning = match symbol.to_lowercase().as_str() {
                "water" => SymbolMeaning {
                    symbol: symbol.to_string(),
                    meaning: "Represents the unconscious, emotion, or the flow of life".to_string(),
                    contexts: vec!["Jungian".to_string(), "Universal".to_string()],
                },
                "fire" => SymbolMeaning {
                    symbol: symbol.to_string(),
                    meaning: "Transformation, passion, or destruction and rebirth".to_string(),
                    contexts: vec!["Alchemical".to_string(), "Universal".to_string()],
                },
                "snake" => SymbolMeaning {
                    symbol: symbol.to_string(),
                    meaning: "Renewal, transformation, healing, or kundalini energy".to_string(),
                    contexts: vec!["Jungian".to_string(), "Mythological".to_string()],
                },
                "falling" => SymbolMeaning {
                    symbol: symbol.to_string(),
                    meaning: "Loss of control, anxiety, or letting go".to_string(),
                    contexts: vec!["Psychological".to_string()],
                },
                "flying" => SymbolMeaning {
                    symbol: symbol.to_string(),
                    meaning: "Freedom, transcendence, or escape from limitations".to_string(),
                    contexts: vec!["Psychological".to_string(), "Spiritual".to_string()],
                },
                // Add more symbols as needed
                _ => SymbolMeaning {
                    symbol: symbol.to_string(),
                    meaning: "No standard meaning found in local database".to_string(),
                    contexts: vec![],
                },
            };

            results.push(meaning);
        }

        Ok(results)
    }

    async fn list_tools_impl(&self) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
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
                Err(format!("Failed to list MCP tools: {}", e).into())
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextOperation {
    Count,    // Count characters, words, etc.
    Validate, // Check if text matches certain criteria
    Format,   // Format or clean text
    Analyze,  // Analyze sentiment or content
}

impl TextOperation {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "count" => Ok(Self::Count),
            "validate" => Ok(Self::Validate),
            "format" => Ok(Self::Format),
            "analyze" => Ok(Self::Analyze),
            _ => Err(format!(
                "Unknown operation: {}. Supported operations: count, validate, format, analyze",
                s
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolMeaning {
    pub symbol: String,
    pub meaning: String,
    pub contexts: Vec<String>,
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
