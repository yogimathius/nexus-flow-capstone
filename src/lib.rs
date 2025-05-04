// Re-export modules
pub mod api;
pub mod config;
pub mod db;
pub mod logging;
pub mod models;
pub mod server;
pub mod services;

// Re-export the Server struct for easy access
pub use server::Server;

// Re-export important clients for convenience
pub use services::ai_client::AiClient;
pub use services::mcp_client::{McpClient, McpClientTrait};

use sqlx::PgPool;

// Application state shared with handlers
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub ai_client: AiClient,
}

impl AppState {
    pub fn new(db_pool: PgPool, api_key: String, github_token: Option<String>) -> Self {
        // Create AI client without MCP initially
        let ai_client = AiClient::new(api_key, github_token);

        AppState { db_pool, ai_client }
    }

    // Adds MCP client to the app state by connecting it to the AI client
    pub async fn with_mcp_client(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Initialize MCP client
        let mcp_client = McpClient::new().await?;

        // Verify MCP client is actually working by testing client capabilities
        // We just need to make sure the client is connected, not that specific tools work
        let _client_capabilities = mcp_client.list_tools().await?;

        // If we get here, the client is working

        // Connect MCP client to AI client
        self.ai_client = self.ai_client.clone().with_mcp_client(mcp_client);

        Ok(())
    }
}
