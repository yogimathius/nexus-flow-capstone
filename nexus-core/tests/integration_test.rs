mod setup;

use axum_server::AppState;

#[tokio::test]
#[ignore] // Skip by default as it requires real services
async fn test_app_state_with_mcp_client() {
    setup::initialize();

    // Create a mock DB pool (real one would be created from a connection string)
    let db_pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres")
        .await
        .expect("Failed to connect to database");

    // Create app state with basic config
    let app_state = AppState::new(
        db_pool,
        "test_api_key".to_string(),
        Some("test_github_token".to_string()),
    );

    // Verify AI client exists but has no MCP client yet
    assert!(app_state.ai_client.has_mcp_client() == false);

    // This would connect to a real MCP client - skip actual connection
    // let mut app_state_with_mcp = app_state.clone();
    // app_state_with_mcp.with_mcp_client().await.unwrap();
    // assert!(app_state_with_mcp.ai_client.has_mcp_client() == true);
}
