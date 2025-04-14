use axum::{
    Router,
    extract::Json,
    http::{Method, StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use tower_http::cors::{Any, CorsLayer};

use crate::ai_client::OpenRouterClient;
use crate::mcp_client::{McpClient, TextOperation};

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

    pub fn router(&self) -> Router {
        // Create CORS layer with explicitly specified headers
        let cors = CorsLayer::new()
            // Allow any origin
            .allow_origin(tower_http::cors::Any)
            // Allow common methods
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            // Explicitly specify the allowed headers instead of Any
            .allow_headers([
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
                header::ORIGIN,
            ]);

        // Define routes
        let app = Router::new()
            .route("/health", axum::routing::get(health_handler))
            .route(
                "/api/process_text",
                axum::routing::post(process_text_handler),
            )
            .route(
                "/api/interpret_dream",
                axum::routing::post(interpret_dream_handler),
            )
            // Apply CORS as the first middleware
            .layer(cors);

        println!("Router configured with CORS middleware and explicit headers");

        app
    }

    pub async fn run(&self) {
        let addr = format!("0.0.0.0:{}", self.port);
        println!("Binding server to address: {}", addr);

        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(listener) => {
                println!("Successfully bound to address: {}", addr);
                listener
            }
            Err(e) => {
                eprintln!("Failed to bind to address {}: {}", addr, e);
                return;
            }
        };

        let app = self.router();

        // Print available endpoints
        self.print_available_endpoints();

        // Start the server
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

// Handler functions

async fn health_handler() -> &'static str {
    "ok"
}

async fn process_text_handler(Json(payload): Json<TextRequest>) -> impl IntoResponse {
    println!(
        "Process text handler called with operation: {}",
        payload.operation
    );

    // Parse the operation
    let operation = match TextOperation::from_str(&payload.operation) {
        Ok(op) => op,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(TextResponse {
                    result: e,
                    status: "error".to_string(),
                }),
            );
        }
    };

    // Initialize MCP client
    match McpClient::new().await {
        Ok(client) => match client.process_text(&payload.text, operation).await {
            Ok(result) => (
                StatusCode::OK,
                Json(TextResponse {
                    result,
                    status: "success".to_string(),
                }),
            ),
            Err(e) => {
                eprintln!("Error processing text: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(TextResponse {
                        result: format!("Could not process text: {}", e),
                        status: "error".to_string(),
                    }),
                )
            }
        },
        Err(e) => {
            eprintln!("Failed to initialize MCP client: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TextResponse {
                    result: "Failed to initialize text processor".to_string(),
                    status: "error".to_string(),
                }),
            )
        }
    }
}

async fn interpret_dream_handler(Json(payload): Json<DreamRequest>) -> impl IntoResponse {
    println!("Dream handler called with text: {}", payload.dream_text);

    // Initialize AI client
    let api_key = match std::env::var("OPENROUTER_API_KEY") {
        Ok(key) => {
            println!("Successfully loaded API key with length: {}", key.len());
            key
        }
        Err(e) => {
            eprintln!("Failed to load API key: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DreamResponse {
                    interpretation: format!("Error: Could not load API key: {:?}", e),
                    status: "error".to_string(),
                }),
            );
        }
    };

    println!("Initializing AI client with API key");
    let ai_client = OpenRouterClient::new(api_key);

    // Request dream interpretation
    println!("Sending dream text to OpenRouter...");
    match ai_client
        .request_dream_interpretation(&payload.dream_text)
        .await
    {
        Ok(interpretation) => {
            println!("Successfully received interpretation");
            (
                StatusCode::OK,
                Json(DreamResponse {
                    interpretation,
                    status: "success".to_string(),
                }),
            )
        }
        Err(e) => {
            eprintln!("Error interpreting dream: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DreamResponse {
                    interpretation: format!("Could not interpret dream: {:?}", e),
                    status: "error".to_string(),
                }),
            )
        }
    }
}
