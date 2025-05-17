mod mocks;
mod setup;

use serde_json::json;
use wiremock::matchers::{any, method};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Test dream interpretation with mocked services
#[tokio::test]
async fn test_ai_client_with_mocks() {
    // Set up a mock server for HTTP requests
    let mock_server = MockServer::start().await;

    // Create a mock response for the OpenRouter API
    let mock_response = json!({
        "choices": [
            {
                "message": {
                    "role": "assistant",
                    "content": "This dream symbolizes freedom and transcendence..."
                }
            }
        ]
    });

    // Configure the mock server - use any() matcher since the complete path depends on the server
    Mock::given(method("POST"))
        .and(any())
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    // Create an AI client using the mock URL
    let client = setup::create_test_ai_client_with_url(&mock_server.uri());

    // Test dream text
    let dream_text = "I was flying over a city made of crystal.";

    // Request interpretation
    let result = client.request_dream_interpretation(dream_text).await;

    // We expect the mock to return a proper response
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
            panic!("Expected successful response but got error: {}", e);
        }
    }
}

/// Test handling of error responses from API
#[tokio::test]
async fn test_ai_client_handles_api_error() {
    // Set up a mock server for HTTP requests
    let mock_server = MockServer::start().await;

    // Configure the mock server to return an error - use any() matcher since the complete path depends on the server
    Mock::given(method("POST"))
        .and(any())
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "error": "API key invalid"
        })))
        .mount(&mock_server)
        .await;

    // Create an AI client with mock URL
    let client = setup::create_test_ai_client_with_url(&mock_server.uri());

    // Test dream text
    let dream_text = "I was flying over a city made of crystal.";

    // Request interpretation should return an error
    let result = client.request_dream_interpretation(dream_text).await;

    // We expect an error
    match result {
        Ok(_) => {
            panic!("Expected error but got success");
        }
        Err(e) => {
            println!("Got error as expected: {}", e);
            // Any kind of error is okay for this test
        }
    }
}
