use rmcp::{ServiceExt, object, transport::TokioChildProcess};
use serde_json;
use std::error::Error;
use tokio::process::Command;
use tracing::{debug, error, info};

pub struct McpClient {
    client: rmcp::service::RunningService<rmcp::RoleClient, ()>,
}

impl McpClient {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        info!("Initializing MCP client...");

        // Initialize the MCP client with the server-everything implementation
        let mut command = Command::new("npx");
        command
            .arg("-y")
            .arg("@modelcontextprotocol/server-everything");

        debug!("Starting MCP server with command: {:?}", command);

        let client = match ().serve(TokioChildProcess::new(&mut command)?).await {
            Ok(client) => {
                info!("MCP client successfully initialized");
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

    // Process text: analyze, transform, or enhance text
    pub async fn process_text(
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
