pub mod dream;
pub mod health;
pub mod validation;

use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

use self::dream::{
    create_dream_handler, get_dream_handler, interpret_dream_handler, list_dreams_handler,
};
use self::health::health_handler;

// Create a router with all API routes
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/api/interpret_dream", post(interpret_dream_handler))
        .route("/api/dreams", post(create_dream_handler))
        .route("/api/dreams", get(list_dreams_handler))
        .route("/api/dreams/{id}", get(get_dream_handler))
        .with_state(state)
}
