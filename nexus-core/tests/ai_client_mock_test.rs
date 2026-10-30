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
        .expect(1)
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
            assert_eq!(interpretation, "This dream symbolizes freedom and transcendence...");
        }
        Err(e) => {
            panic!("Expected successful response but got error: {}", e);
        }
    }
}

/// Test handling of error responses from API
#[tokio::test]
async fn test_ai_client_handles_api_error() {
    // Create an AI client with an empty token to simulate an error case
    let client = Box::new(axum_server::services::ai::AiClient::new(Some(
        "".to_string(),
    ))) as Box<dyn axum_server::services::ai::AiClientTrait>;

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
            // An absent provider must not produce a successful interpretation.
            assert!(
                format!("{}", e).contains("No AI provider configured"),
                "Error should mention the missing provider: {}",
                e
            );
        }
    }
}
