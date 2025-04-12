use rmcp::{ServiceExt, object, transport::TokioChildProcess};
use serde_json;
use std::error::Error;
use tokio::process::Command;

pub struct McpClient {
    client: rmcp::service::RunningService<rmcp::RoleClient, ()>,
}

impl McpClient {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // Initialize the MCP client with the server-everything implementation
        let client = ()
            .serve(TokioChildProcess::new(
                Command::new("npx")
                    .arg("-y")
                    .arg("@modelcontextprotocol/server-everything"),
            )?)
            .await?;

        Ok(Self { client })
    }

    pub async fn interpret_dream(
        &self,
        dream_text: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Create the prompt
        let prompt = format!("Please interpret this dream:\n\n{}", dream_text);

        // Use the call_tool method to interact with a completion tool
        // This is similar to how your examples use tools
        let tool_result = self
            .client
            .call_tool(rmcp::model::CallToolRequestParam {
                name: "completion".into(),
                arguments: Some(object!({
                    "model": "gpt-3.5-turbo",
                    "temperature": 0.7,
                    "max_tokens": 500,
                    "messages": [{
                        "role": "user",
                        "content": prompt
                    }]
                })),
            })
            .await?;

        // Extract the result from the tool output
        // Debug: print the raw structure
        println!("Tool result: {:#?}", tool_result);

        // Convert the tool result to a JSON value we can work with
        let json_result = serde_json::to_value(&tool_result)?;
        println!("JSON result: {}", json_result);

        // Try to extract the response content
        if let Some(choices) = json_result.get("choices") {
            if let Some(first_choice) = choices.get(0) {
                if let Some(message) = first_choice.get("message") {
                    if let Some(content) = message.get("content") {
                        if let Some(text) = content.as_str() {
                            return Ok(text.to_string());
                        }
                    }
                }
            }
        }

        // Try another common structure pattern
        if let Some(output) = json_result.get("output") {
            if let Some(content) = output.get("content") {
                if let Some(text) = content.as_str() {
                    return Ok(text.to_string());
                }
            }
        }

        // If we couldn't extract the content through the expected paths
        Err("Could not extract content from response".into())
    }
}
