use crate::error::{AppError, AppResult};
use crate::services::mcp::types::SymbolMeaning;
use rmcp::model::CallToolRequestParam;
use serde_json;
use tracing::debug;

/// Provides functionality for symbol lookup operations
pub struct SymbolLookup;

impl SymbolLookup {
    /// Attempt to look up symbols via the MCP server
    pub async fn try_mcp_lookup(
        client: &rmcp::service::RunningService<
            rmcp::RoleClient,
            rmcp::model::InitializeRequestParam,
        >,
        symbols: &[&str],
    ) -> AppResult<Vec<SymbolMeaning>> {
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
                client.call_tool(CallToolRequestParam {
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

        // Try fetching general symbols if specific lookup failed
        Self::try_fetch_general_symbols(client).await
    }

    /// Try to fetch general symbols from the MCP server
    async fn try_fetch_general_symbols(
        client: &rmcp::service::RunningService<
            rmcp::RoleClient,
            rmcp::model::InitializeRequestParam,
        >,
    ) -> AppResult<Vec<SymbolMeaning>> {
        debug!("No specific symbols found, trying general symbols API");

        let params = serde_json::json!({
            "limit": 10
        });

        match tokio::time::timeout(
            std::time::Duration::from_secs(5),
            client.call_tool(CallToolRequestParam {
                name: "get_symbols".into(),
                arguments: params.as_object().cloned(),
            }),
        )
        .await
        {
            Ok(Ok(result)) => {
                debug!("Successfully retrieved general symbols");
                let mut results = Vec::new();

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
                Err(AppError::McpError(format!(
                    "MCP symbol lookup tool error: {}",
                    e
                )))
            }
            Err(_) => {
                debug!("MCP get_symbols tool timed out");
                Err(AppError::McpError(
                    "MCP symbol lookup timed out".to_string(),
                ))
            }
        }
    }

    /// Lookup symbols in a local database when MCP is unavailable
    pub fn lookup_locally(symbols: Vec<&str>) -> AppResult<Vec<SymbolMeaning>> {
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
}
