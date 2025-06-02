use crate::error::{AppError, AppResult};
use crate::services::mcp::types::TextOperation;
use rmcp::model::CallToolRequestParam;
use rmcp::object;
use serde_json;
use tracing::debug;

pub async fn try_mcp_processing(
    client: &rmcp::service::RunningService<rmcp::RoleClient, rmcp::model::InitializeRequestParam>,
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
