use axum::http::{HeaderValue, Method};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::AppState;
use crate::api;
use crate::db;

// API Types
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

// Server implementation
pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }

    pub fn router(&self) -> axum::Router {
        // Create CORS layer
        let cors = CorsLayer::new()
            // Allow origins from the frontend
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_origin("https://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_origin("http://127.0.0.1:3004".parse::<HeaderValue>().unwrap())
            .allow_origin("http://127.0.0.1:3001".parse::<HeaderValue>().unwrap())
            // Allow common methods
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            // Allow common headers
            .allow_headers(Any);

        // Placeholder for state - this would be created from real config in run()
        let app_state = AppState {
            db_pool: sqlx::PgPool::connect_lazy("postgresql://").unwrap(),
            ai_client: crate::AiClient::new("dummy_key".to_string(), None),
        };

        // Apply API routes with CORS
        api::router(app_state).layer(cors)
    }

    pub async fn run(&self) {
        // Load configuration
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

        // Create or check database
        match db::migrate::create_database_if_not_exists(&config.database_url).await {
            Ok(_) => info!("Database checked/created successfully"),
            Err(e) => {
                tracing::error!("Database setup error: {}", e);
                return;
            }
        }

        // Connect to database
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

        // Run migrations
        info!("Running database migrations...");
        if let Err(e) = db::migrate::run_migrations(&db_pool).await {
            tracing::error!("Migration error: {}", e);
            return;
        }
        info!("Migrations complete!");

        // Create application state
        let mut app_state = AppState::new(db_pool, config.openrouter_api_key, config.github_token);

        // Try to initialize MCP client and connect it to AI client if enabled
        if config.enable_mcp {
            info!("MCP client integration is enabled, attempting to connect...");

            // Set MCP server URL in environment if provided
            if let Some(url) = &config.mcp_server_url {
                unsafe {
                    std::env::set_var("MCP_SERVER_URL", url);
                }
            }

            if let Err(e) = app_state.with_mcp_client().await {
                tracing::warn!(
                    "Failed to initialize or verify MCP client connection: {}. Continuing without MCP functionality. \
                     Make sure the MCP server is running at the expected endpoint.",
                    e
                );
            } else {
                info!("MCP client successfully initialized and verified connection");
            }
        } else {
            info!("MCP client integration is disabled by configuration");
        }

        // Create the router with app state
        let app = api::router(app_state);

        // Add CORS
        let cors = CorsLayer::new()
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_origin("https://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_origin("http://127.0.0.1:3004".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers(Any);

        // Add trace layer for HTTP request/response logging
        let app = app
            .layer(cors)
            .layer(tower_http::trace::TraceLayer::new_for_http());

        // Bind to address
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        info!("Starting server on {}", addr);

        // Start the server
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap_or_else(|e| {
            tracing::error!("Server error: {:?}", e);
        });
    }
}
