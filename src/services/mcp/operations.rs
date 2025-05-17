use crate::error::{AppError, AppResult};
use crate::services::mcp::types::TextOperation;
use rmcp::model::CallToolRequestParam;
use rmcp::{ServiceExt, object};
use serde_json;
use tracing::{debug, info};

/// Functions for text processing operations
pub struct TextProcessor;

impl TextProcessor {
    /// Process text using an external MCP server
    pub async fn try_mcp_processing(
        client: &rmcp::service::RunningService<
            rmcp::RoleClient,
            rmcp::model::InitializeRequestParam,
        >,
        text: &str,
        operation: &TextOperation,
    ) -> AppResult<String> {
        // Try to use MCP for processing
        let op_name = match operation {
            TextOperation::Count => "text_count",
            TextOperation::Validate => "text_validate",
            TextOperation::Format => "text_format",
            TextOperation::Analyze => "text_analyze",
        };

        debug!("Attempting to use MCP tool: {}", op_name);

        match client
            .call_tool(CallToolRequestParam {
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
                Err(AppError::McpError(format!(
                    "MCP tool '{}' not available",
                    op_name
                )))
            }
        }
    }

    /// Process text locally when MCP server is unavailable
    pub fn process_locally(text: &str, operation: TextOperation) -> AppResult<String> {
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
}
