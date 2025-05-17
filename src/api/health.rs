use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

// Health check handler with standardized response format
pub async fn health_handler() -> impl IntoResponse {
    Json(HealthResponse {
        status: "success".to_string(),
        message: "Service is running normally".to_string(),
    })
}
