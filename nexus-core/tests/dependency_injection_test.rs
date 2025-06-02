mod mock_ai_client;

use crate::mock_ai_client::MockAiClient;
use axum_server::{AiClientTrait, AppState};
use sqlx::PgPool;
use std::sync::Arc;

#[tokio::test]
async fn test_app_state_with_mock_ai_client() {
    // Create a mock AI client that will always succeed with a predefined response
    let mock_ai_client = MockAiClient::new(true, "This is a mock dream interpretation");

    // Create the app state with our mock
    let app_state = AppState {
        // Use a mock/empty PgPool for testing
        db_pool: PgPool::connect_lazy("postgres://postgres:password@localhost/postgres").unwrap(),
        ai_client: Arc::new(Box::new(mock_ai_client) as Box<dyn AiClientTrait>),
    };

    // Test that our mock client works through the app state
    let result = app_state
        .ai_client
        .request_dream_interpretation("Test dream")
        .await;

    assert!(result.is_ok());
    let interpretation = result.unwrap();

    // Check that the mock client returned our predefined response
    assert!(interpretation.contains("This is a mock dream interpretation"));
    assert!(interpretation.contains("Test dream"));

    // Check that our mock correctly reports no MCP client
    assert!(!app_state.ai_client.has_mcp_client());
}

// This test demonstrates that we can easily swap implementations
#[tokio::test]
async fn test_app_state_with_failing_mock_ai_client() {
    // Create a mock AI client that will always fail
    let mock_ai_client = MockAiClient::new(false, "This shouldn't be returned");

    // Create the app state with our mock
    let app_state = AppState {
        // Use a mock/empty PgPool for testing
        db_pool: PgPool::connect_lazy("postgres://postgres:password@localhost/postgres").unwrap(),
        ai_client: Arc::new(Box::new(mock_ai_client) as Box<dyn AiClientTrait>),
    };

    // Test that our mock client correctly returns an error
    let result = app_state
        .ai_client
        .request_dream_interpretation("Test dream")
        .await;

    // Check that the result is an error
    assert!(result.is_err());
}
