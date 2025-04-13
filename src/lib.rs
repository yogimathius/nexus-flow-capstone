pub mod ai_client;
pub mod mcp_client;

use axum::{
    Router,
    extract::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use mcp_client::{McpClient, TextOperation};

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

// Define a new request type for dream interpretation
#[derive(Deserialize)]
pub struct DreamRequest {
    pub dream_text: String,
}

// Define a new response type for dream interpretation
#[derive(Serialize)]
pub struct DreamResponse {
    pub interpretation: String,
    pub status: String,
}

// Create a function to build and return the Router
pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route(
            "/api/process_text",
            post(|Json(payload): Json<TextRequest>| async move {
                // Parse the operation
                let operation = match TextOperation::from_str(&payload.operation) {
                    Ok(op) => op,
                    Err(e) => {
                        return Json(TextResponse {
                            result: e,
                            status: "error".to_string(),
                        });
                    }
                };

                // Initialize MCP client
                match McpClient::new().await {
                    Ok(client) => match client.process_text(&payload.text, operation).await {
                        Ok(result) => Json(TextResponse {
                            result,
                            status: "success".to_string(),
                        }),
                        Err(e) => {
                            eprintln!("Error processing text: {:?}", e);
                            Json(TextResponse {
                                result: format!("Could not process text: {}", e),
                                status: "error".to_string(),
                            })
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to initialize MCP client: {:?}", e);
                        Json(TextResponse {
                            result: "Failed to initialize text processor".to_string(),
                            status: "error".to_string(),
                        })
                    }
                }
            }),
        )
        .route(
            "/api/interpret_dream",
            post(|Json(payload): Json<DreamRequest>| async move {
                // Initialize AI client
                let api_key = std::env::var("OPENROUTER_API_KEY").expect("API key not set");
                let ai_client = ai_client::OpenRouterClient::new(api_key);

                // Request dream interpretation
                match ai_client
                    .request_dream_interpretation(&payload.dream_text)
                    .await
                {
                    Ok(interpretation) => Json(DreamResponse {
                        interpretation,
                        status: "success".to_string(),
                    }),
                    Err(e) => {
                        eprintln!("Error interpreting dream: {:?}", e);
                        Json(DreamResponse {
                            interpretation: format!("Could not interpret dream: {:?}", e),
                            status: "error".to_string(),
                        })
                    }
                }
            }),
        )
}
