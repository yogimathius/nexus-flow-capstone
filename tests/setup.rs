use axum_server::{AiClient, Server};
use dotenv::dotenv;
use sqlx::PgPool;
use std::sync::Once;

// Make sure .env is loaded only once across all tests
static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        dotenv().ok();

        // Initialize tracing for testing
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info,rmcp=debug")
            .try_init();
    });
}

// Create a test server instance with a specific port
pub fn create_test_server() -> Server {
    initialize();
    Server::new(0) // Port 0 = random available port
}

// Create a connection pool for testing
pub async fn create_test_db_pool() -> PgPool {
    initialize();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

    PgPool::connect(&database_url)
        .await
        .expect("Failed to create database connection pool")
}

// Helper to create a mock AI client
pub fn create_test_ai_client() -> AiClient {
    initialize();

    let api_key =
        std::env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| "test_api_key".to_string());

    let github_token = std::env::var("GITHUB_TOKEN").ok();

    AiClient::new(api_key, github_token)
}

// Helper to create a mock AI client with a custom URL (for tests with wiremock)
#[cfg(test)]
pub fn create_test_ai_client_with_url(url: &str) -> AiClient {
    initialize();

    AiClient::new("test_api_key".to_string(), None).with_openrouter_url(url)
}
