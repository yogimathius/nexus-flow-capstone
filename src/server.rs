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

// Helper function to create consistent CORS configuration
fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost:3004".parse::<HeaderValue>().unwrap())
        .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap())
        .allow_origin("http://127.0.0.1:3001".parse::<HeaderValue>().unwrap())
        .allow_origin("http://127.0.0.1:3004".parse::<HeaderValue>().unwrap())
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

    pub fn router(&self) -> axum::Router {
        let app_state = AppState {
            db_pool: sqlx::PgPool::connect_lazy("postgresql://").unwrap(),
            ai_client: std::sync::Arc::new(Box::new(crate::AiClient::new(
                "dummy_key".to_string(),
                None,
            )) as Box<dyn crate::AiClientTrait>),
        };
        api::router(app_state).layer(create_cors_layer())
    }

    pub async fn run(&self) {
        let config = match crate::config::Config::from_env() {
            Ok(config) => {
                info!("Configuration loaded successfully");
                config
            }
            Err(e) => {
                tracing::error!("Failed to load configuration: {}", e);
                return;
            }
        };

        info!("config: {:?}", config);

        match db::migrate::create_database_if_not_exists(&config.database_url).await {
            Ok(_) => info!("Database checked/created successfully"),
            Err(e) => {
                tracing::error!("Database setup error: {}", e);
                return;
            }
        }

        let db_pool = match db::create_pool(&config.database_url).await {
            Ok(pool) => {
                info!("Connected to database successfully");
                pool
            }
            Err(e) => {
                tracing::error!("Failed to connect to database: {}", e);
                return;
            }
        };

        info!("Running database migrations...");
        if let Err(e) = db::migrate::run_migrations(&db_pool).await {
            tracing::error!("Migration error: {}", e);
            return;
        }
        info!("Migrations complete!");

        let mut app_state = AppState::new(db_pool, config.openrouter_api_key, config.github_token);

        if config.enable_mcp {
            info!("MCP client integration is enabled, attempting to connect...");

            if let Some(url) = &config.mcp_server_url {
                unsafe {
                    std::env::set_var("MCP_SERVER_URL", url);
                }
            }

            match app_state.with_mcp_client().await {
                Ok(_) => {
                    info!("MCP client successfully initialized and verified connection");
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to initialize or verify MCP client connection: {}. Continuing without MCP functionality. \
                         Make sure the MCP server is running at the expected endpoint.",
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

        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        info!("Starting server on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap_or_else(|e| {
            tracing::error!("Server error: {:?}", e);
        });
    }
}
