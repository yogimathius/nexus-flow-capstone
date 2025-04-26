use axum::http::{HeaderValue, Method};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use tower_http::cors::{Any, CorsLayer};

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
                println!("Configuration loaded successfully");
                config
            }
            Err(e) => {
                eprintln!("Failed to load configuration: {}", e);
                return;
            }
        };

        println!("config: {:?}", config);

        // Create or check database
        match db::migrate::create_database_if_not_exists(&config.database_url).await {
            Ok(_) => println!("Database checked/created successfully"),
            Err(e) => {
                eprintln!("Database setup error: {}", e);
                return;
            }
        }

        // Connect to database
        let db_pool = match db::create_pool(&config.database_url).await {
            Ok(pool) => {
                println!("Connected to database successfully");
                pool
            }
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);
                return;
            }
        };

        // Run migrations
        if let Err(e) = db::migrate::run_migrations(&db_pool).await {
            eprintln!("Migration error: {}", e);
            return;
        }

        // Create application state
        let app_state = AppState::new(db_pool, config.openrouter_api_key, config.github_token);

        // Create the router with app state
        let app = api::router(app_state);

        // Add CORS
        let app = app.layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_origin("https://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_origin("http://127.0.0.1:3004".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers(Any),
        );

        // Bind to address
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        println!("Starting server on {}", addr);

        // Start the server
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap_or_else(|e| {
            eprintln!("Server error: {:?}", e);
        });
    }

    fn print_available_endpoints(&self) {
        println!("Available endpoints:");
        println!("  GET  /health - Health check endpoint");
        println!("  POST /api/process_text - Process text with operations");
        println!("  POST /api/interpret_dream - Interpret dreams using AI");
        println!("");
        println!("Example usage:");
        println!(
            "  curl -X POST http://localhost:{}/api/process_text \\",
            self.port
        );
        println!("    -H \"Content-Type: application/json\" \\");
        println!(
            "    -d '{{\"text\": \"this is a sample text. it needs formatting!\", \"operation\": \"format\"}}'"
        );
        println!("");
        println!("Dream interpretation example:");
        println!(
            "  curl -v -X POST http://localhost:{}/api/interpret_dream \\",
            self.port
        );
        println!("    -H \"Content-Type: application/json\" \\");
        println!("    -d '{{\"dream_text\": \"I was flying over a city made of crystal.\"}}'");
        println!("");
        println!("Available operations:");
        println!("  count    - Count characters, words, and lines");
        println!("  validate - Check text for validity and patterns");
        println!("  format   - Format and clean text (capitalize, etc.)");
        println!("  analyze  - Analyze text sentiment");
        println!("");
    }
}
