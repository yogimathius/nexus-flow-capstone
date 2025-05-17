mod setup;

use axum_server::AppError;

// Test the AI client with a mock
#[tokio::test]
#[ignore] // Skip because it requires API keys
async fn test_ai_client_basic() {
    // Initialize test setup
    setup::initialize();

    // Create a test AI client
    let client = setup::create_test_ai_client();

    // Test dream text
    let dream_text = "I was flying over a city made of crystal.";

    // Request interpretation
    let result = client.request_dream_interpretation(dream_text).await;

    // We can't guarantee the exact response, but we can check that it's not an error
    // or we might get an error if the API key is invalid or rate limited
    match result {
        Ok(interpretation) => {
            // Check that we got a non-empty interpretation
            assert!(
                !interpretation.is_empty(),
                "Interpretation should not be empty"
            );
            println!("Test succeeded with interpretation: {}", interpretation);
        }
        Err(e) => {
            // If we get an error, it should be a reasonable error
            match e {
                AppError::ApiKeyError(_)
                | AppError::RateLimitedError
                | AppError::ExternalServiceError(_) => {
                    // These errors are acceptable in a test environment
                    println!("Got expected error during testing: {:?}", e);
                }
                unexpected_error => {
                    panic!("Unexpected error from AI client: {:?}", unexpected_error);
                }
            }
        }
    }
}

// Test that we handle empty responses correctly
#[tokio::test]
#[ignore] // Needs mocking to work consistently
async fn test_ai_client_handles_empty_response() {
    // This would need a proper mock of the OpenRouter API
    // We'll leave it as a placeholder for now
}

// Test that we handle API errors correctly
#[tokio::test]
#[ignore] // Needs mocking to work consistently
async fn test_ai_client_handles_api_error() {
    // This would need a proper mock of the OpenRouter API
    // We'll leave it as a placeholder for now
}
