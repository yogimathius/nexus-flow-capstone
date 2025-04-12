pub mod ai_client;
pub mod mcp_client;

use axum::{
    Router,
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::env;

use ai_client::OpenRouterClient;

// API Types
#[derive(Deserialize)]
pub struct DreamRequest {
    pub dream_text: String,
}

#[derive(Serialize)]
pub struct DreamResponse {
    pub interpretation: String,
    pub status: String,
}

// Create a function to build and return the Router (for testing)
pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/interpret_dream", post(interpret_dream))
}

// Handler functions
pub async fn health_check() -> impl IntoResponse {
    "ok"
}

pub async fn interpret_dream(Json(payload): Json<DreamRequest>) -> Json<DreamResponse> {
    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");
    let client = OpenRouterClient::new(api_key);

    match client
        .request_dream_interpretation(&payload.dream_text)
        .await
    {
        Ok(interpretation) => Json(DreamResponse {
            interpretation,
            status: "success".to_string(),
        }),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            Json(DreamResponse {
                interpretation: "Could not process dream interpretation".to_string(),
                status: "error".to_string(),
            })
        }
    }
}
