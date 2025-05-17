use axum_server::Server;
use axum_server::services::ai::{AiClient, AiClientTrait};
use dotenv::dotenv;
use sqlx::PgPool;
use std::sync::Once;

// Make sure .env is loaded only once across all tests
#[allow(dead_code)]
static INIT: Once = Once::new();

#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn create_test_server() -> Server {
    initialize();
    Server::new(0) // Port 0 = random available port
}

// Create a connection pool for testing
#[allow(dead_code)]
pub async fn create_test_db_pool() -> PgPool {
    initialize();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

    PgPool::connect(&database_url)
        .await
        .expect("Failed to create database connection pool")
}

// Helper to create a mock AI client
#[allow(dead_code)]
pub fn create_test_ai_client() -> Box<dyn AiClientTrait> {
    initialize();

    let github_token = std::env::var("GITHUB_TOKEN").ok();

    Box::new(AiClient::new(github_token))
}

// Helper to create a mock AI client with a custom URL (for tests with wiremock)
#[allow(dead_code)]
pub fn create_test_ai_client_with_url(_url: &str) -> Box<dyn AiClientTrait> {
    initialize();

    // Just create a test client with a mock GitHub token
    let github_token = Some("test_github_token".to_string());

    // For tests, we'll just use a mock client since we don't have OpenRouter anymore
    println!("Creating test AI client with mock GitHub token");

    Box::new(AiClient::new(github_token))
}
