use axum::http::{HeaderValue, Method};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::AppState;
use crate::api;
use crate::db;

#[derive(Deserialize)]
pub struct TextRequest {
    pub text: String,
    pub operation: String,
}

#[derive(Serialize)]
pub struct TextResponse {
    pub result: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct DreamRequest {
    pub dream_text: String,
}

#[derive(Serialize)]
pub struct DreamResponse {
    pub interpretation: String,
    pub status: String,
}

pub struct Server {
    port: u16,
}

#[cfg(test)]
mod tests {
    use super::Server;

    #[test]
    fn server_retains_requested_port() {
        assert_eq!(Server::new(3017).port(), 3017);
    }
}

// Helper function to create consistent CORS configuration
fn create_cors_layer() -> CorsLayer {
    let mut allowed_origins = vec![
        "http://localhost:3000".to_string(),
        "http://localhost:3001".to_string(),
        "http://localhost:3004".to_string(),
        "http://127.0.0.1:3000".to_string(),
        "http://127.0.0.1:3001".to_string(),
        "http://127.0.0.1:3004".to_string(),
    ];

    // Add production origin if it exists and is valid
    if let Ok(prod_origin) = std::env::var("PROD_ORIGIN") {
        if !prod_origin.is_empty() && prod_origin.starts_with("http") {
            allowed_origins.push(prod_origin);
        }
    }

    // Convert to HeaderValues and filter out any invalid ones
    let header_values: Vec<HeaderValue> = allowed_origins
        .into_iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    info!("CORS configured for origins: {:?}", header_values);

    CorsLayer::new()
        .allow_origin(header_values)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn router(&self) -> axum::Router {
        let app_state = AppState {
            db_pool: sqlx::PgPool::connect_lazy("postgresql://").unwrap(),
            ai_client: std::sync::Arc::new(
                Box::new(crate::AiClient::new(None)) as Box<dyn crate::AiClientTrait>
            ),
        };
        api::router(app_state).layer(create_cors_layer())
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = crate::config::Config::from_env()?;
        info!("Configuration loaded successfully");

        db::migrate::create_database_if_not_exists(&config.database_url).await?;
        info!("Database checked/created successfully");

        let db_pool = db::create_pool(&config.database_url).await?;
        info!("Connected to database successfully");
        db::migrate::run_migrations(&db_pool).await?;

        let mut app_state = AppState::new(db_pool, config.openrouter_api_key, config.github_token);

        if config.enable_mcp {
            info!("MCP client integration is enabled, attempting to connect...");

            match app_state.with_mcp_client().await {
                Ok(_) => {
                    info!("MCP client successfully initialized and verified connection");
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to initialize or verify MCP client connection: {}. Continuing without MCP functionality. \
                                Check the configured MCP transport and server process.",
                        e
                    );
                }
            }
        } else {
            info!("MCP client integration is disabled by configuration");
        }

        let app = api::router(app_state);

        let app = app
            .layer(create_cors_layer())
            .layer(tower_http::trace::TraceLayer::new_for_http());

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        info!("Starting server on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}
