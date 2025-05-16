pub mod api;
pub mod config;
pub mod db;
pub mod error;
pub mod logging;
pub mod models;
pub mod server;
pub mod services;

pub use server::Server;

pub use error::{AppError, AppResult};
pub use services::ai_client::AiClient;
pub use services::mcp_client::{McpClient, McpClientTrait};
use tracing::{error, info};

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub ai_client: AiClient,
}

impl AppState {
    pub fn new(db_pool: PgPool, api_key: String, github_token: Option<String>) -> Self {
        let ai_client = AiClient::new(api_key, github_token);

        AppState { db_pool, ai_client }
    }

    pub async fn with_mcp_client(&mut self) -> AppResult<()> {
        match McpClient::new().await {
            Ok(mcp_client) => match mcp_client.list_tools().await {
                Ok(_client_capabilities) => {
                    self.ai_client = self.ai_client.clone().with_mcp_client(mcp_client);
                    info!("MCP client successfully connected and enabled");
                    Ok(())
                }
                Err(e) => {
                    error!("MCP client connected but failed to list tools: {:?}", e);
                    Err(AppError::McpError(format!(
                        "MCP client failed to list tools: {}",
                        e
                    )))
                }
            },
            Err(e) => {
                info!(
                    "MCP client initialization failed: {}. Server will run without MCP functionality.",
                    e
                );
                Err(AppError::McpError(format!("{}", e)))
            }
        }
    }
}
