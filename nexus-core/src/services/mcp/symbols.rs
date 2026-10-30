use crate::error::{AppError, AppResult};
use crate::services::mcp::types::SymbolMeaning;
use rmcp::model::{CallToolRequestParam, CallToolResult};
use serde_json::Value;
use tracing::debug;

/// Looks up symbols through the independent Symbols Awakening MCP server.
pub struct SymbolLookup;

impl SymbolLookup {
    pub async fn try_mcp_lookup(
        client: &rmcp::service::RunningService<
            rmcp::RoleClient,
            rmcp::model::InitializeRequestParam,
        >,
        symbols: &[&str],
    ) -> AppResult<Vec<SymbolMeaning>> {
        let mut meanings = Vec::new();

        for symbol in symbols {
            debug!("Searching for symbol '{}' through MCP", symbol);
            let params = serde_json::json!({ "query": symbol, "limit": 5 });
            let response = tokio::time::timeout(
                std::time::Duration::from_secs(3),
                client.call_tool(CallToolRequestParam {
                    name: "search_symbols".into(),
                    arguments: params.as_object().cloned(),
                }),
            )
            .await
            .map_err(|_| AppError::McpError(format!("MCP search timed out for '{symbol}'")))?
            .map_err(|e| AppError::McpError(format!("MCP search failed for '{symbol}': {e}")))?;

            meanings.extend(Self::parse_search_result(&response)?);
        }

        Ok(meanings)
    }

    fn parse_search_result(result: &CallToolResult) -> AppResult<Vec<SymbolMeaning>> {
        if result.is_error == Some(true) {
            return Err(AppError::McpError(
                "MCP search tool reported an error".into(),
            ));
        }

        let text = result
            .content
            .iter()
            .find_map(|content| content.as_text().map(|text| text.text.as_str()))
            .ok_or_else(|| AppError::McpError("MCP search returned no text content".into()))?;
        let payload: Value = serde_json::from_str(text)
            .map_err(|e| AppError::McpError(format!("Invalid MCP search response: {e}")))?;

        if let Some(error) = payload.get("error").and_then(Value::as_str) {
            return Err(AppError::McpError(format!(
                "MCP search tool error: {error}"
            )));
        }

        let symbols = payload
            .get("symbols")
            .and_then(Value::as_array)
            .ok_or_else(|| AppError::McpError("MCP search response has no symbols array".into()))?;

        symbols
            .iter()
            .map(|item| {
                let name = item
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| AppError::McpError("MCP symbol has no name".into()))?;
                let description = item
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or("No description available");
                let contexts = item
                    .get("category")
                    .and_then(Value::as_str)
                    .filter(|value| !value.is_empty())
                    .map(|value| vec![value.to_string()])
                    .unwrap_or_default();

                Ok(SymbolMeaning {
                    symbol: name.to_string(),
                    meaning: description.to_string(),
                    contexts,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::model::Content;

    #[test]
    fn parses_symbols_from_mcp_text_content() {
        let result = CallToolResult::success(vec![Content::text(
            r#"{"symbols":[{"name":"Ouroboros","description":"Cycles and renewal","category":"transformation"}],"count":1}"#,
        )]);

        let symbols = SymbolLookup::parse_search_result(&result).unwrap();
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].symbol, "Ouroboros");
        assert_eq!(symbols[0].meaning, "Cycles and renewal");
        assert_eq!(symbols[0].contexts, vec!["transformation"]);
    }

    #[test]
    fn empty_search_is_not_replaced_with_unrelated_symbols() {
        let result = CallToolResult::success(vec![Content::text(r#"{"symbols":[],"count":0}"#)]);
        assert!(
            SymbolLookup::parse_search_result(&result)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn reports_tool_errors() {
        let result =
            CallToolResult::success(vec![Content::text(r#"{"error":"database unavailable"}"#)]);
        assert!(SymbolLookup::parse_search_result(&result).is_err());
    }
}
