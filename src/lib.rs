// Re-export modules
pub mod api;
pub mod config;
pub mod db;
pub mod models;
pub mod server;
pub mod services;

// Re-export the Server struct for easy access
pub use server::Server;

// Re-export important clients for convenience
pub use services::ai_client::AiClient;
pub use services::mcp_client::McpClient;

use sqlx::PgPool;

// Application state shared with handlers
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub ai_client: AiClient,
}

impl AppState {
    pub fn new(db_pool: PgPool, api_key: String, github_token: Option<String>) -> Self {
        AppState {
            db_pool,
            ai_client: AiClient::new(api_key, github_token),
        }
    }
}
