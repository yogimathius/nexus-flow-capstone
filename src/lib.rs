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
pub use services::ai::{AiClient, AiClientTrait};
pub use services::mcp::client::McpClient;
pub use services::mcp::traits::McpClientTrait;
use tracing::{error, info};

use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub ai_client: Arc<Box<dyn AiClientTrait>>,
}

impl AppState {
    pub fn new(db_pool: PgPool, _api_key: String, github_token: Option<String>) -> Self {
        let ai_client = Arc::new(Box::new(AiClient::new(github_token)) as Box<dyn AiClientTrait>);

        AppState { db_pool, ai_client }
    }

    pub async fn with_mcp_client(&mut self) -> AppResult<()> {
        match McpClient::new().await {
            Ok(mcp_client) => match mcp_client.list_tools().await {
                Ok(_client_capabilities) => {
                    if let Some(concrete_client) =
                        Arc::get_mut(&mut self.ai_client).and_then(|boxed_client| {
                            boxed_client.as_any_mut().downcast_mut::<AiClient>()
                        })
                    {
                        *concrete_client = concrete_client.clone().with_mcp_client(mcp_client);
                        info!("MCP client successfully connected and enabled");
                        Ok(())
                    } else {
                        error!(
                            "Failed to connect MCP client: couldn't downcast to concrete AiClient"
                        );
                        Err(AppError::ApplicationError(
                            "Failed to connect MCP client".into(),
                        ))
                    }
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
