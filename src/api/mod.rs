pub mod dream;
pub mod health;

use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

// Create a router with all API routes
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_handler))
        .route("/api/interpret_dream", post(dream::interpret_dream_handler))
        .route("/api/dreams", post(dream::create_dream_handler))
        .route("/api/dreams", get(dream::list_dreams_handler))
        .route("/api/dreams/{id}", get(dream::get_dream_handler))
        .with_state(state)
}
