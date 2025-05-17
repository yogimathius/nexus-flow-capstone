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

    let api_key =
        std::env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| "test_api_key".to_string());

    let github_token = std::env::var("GITHUB_TOKEN").ok();

    Box::new(AiClient::new(api_key, github_token))
}

// Helper to create a mock AI client with a custom URL (for tests with wiremock)
#[allow(dead_code)]
pub fn create_test_ai_client_with_url(url: &str) -> Box<dyn AiClientTrait> {
    initialize();

    // Create a test client with the given URL for OpenRouter provider
    // Use a more realistic test API key format
    let test_api_key = "sk-or-test-1234567890abcdef1234567890abcdef".to_string();

    // For tests, log the URL we're using
    println!("Creating test AI client with URL: {}", url);

    Box::new(AiClient::new(test_api_key, None).with_openrouter_url(url))
}
