use axum_server::{AiClientTrait, AppError, AppResult};
use std::any::Any;
use std::future::Future;
use std::pin::Pin;

/// MockAiClient - A mock implementation of AiClientTrait for testing
pub struct MockAiClient {
    should_succeed: bool,
    predefined_response: String,
}

impl MockAiClient {
    pub fn new(should_succeed: bool, predefined_response: &str) -> Self {
        MockAiClient {
            should_succeed,
            predefined_response: predefined_response.to_string(),
        }
    }
}

/// Implementation of AiClientTrait for our MockAiClient
impl AiClientTrait for MockAiClient {
    fn clone_box(&self) -> Box<dyn AiClientTrait> {
        Box::new(MockAiClient {
            should_succeed: self.should_succeed,
            predefined_response: self.predefined_response.clone(),
        })
    }

    fn request_dream_interpretation<'a>(
        &'a self,
        dream_text: &'a str,
    ) -> Pin<Box<dyn Future<Output = AppResult<String>> + Send + 'a>> {
        Box::pin(async move {
            if self.should_succeed {
                // Return our predefined response plus the dream text to show we received it
                Ok(format!(
                    "{} (Dream: {})",
                    self.predefined_response, dream_text
                ))
            } else {
                // Simulate an API error
                Err(AppError::ExternalServiceError(
                    "Mock API error for testing".to_string(),
                ))
            }
        })
    }

    fn has_mcp_client(&self) -> bool {
        false // Our mock doesn't have MCP functionality
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_ai_client_success() {
        // Create a mock that will succeed
        let mock_client = MockAiClient::new(true, "This is a test interpretation");
        let client: Box<dyn AiClientTrait> = Box::new(mock_client);

        let result = client.request_dream_interpretation("I was flying").await;

        assert!(result.is_ok());
        let interpretation = result.unwrap();
        assert!(interpretation.contains("This is a test interpretation"));
        assert!(interpretation.contains("I was flying"));
    }

    #[tokio::test]
    async fn test_mock_ai_client_failure() {
        // Create a mock that will fail
        let mock_client = MockAiClient::new(false, "This shouldn't be returned");
        let client: Box<dyn AiClientTrait> = Box::new(mock_client);

        let result = client.request_dream_interpretation("I was falling").await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        // Check that it's the expected error type
        match err {
            AppError::ExternalServiceError(_) => {
                // This is what we expect
            }
            _ => {
                panic!(
                    "Expected ExternalServiceError but got a different error: {:?}",
                    err
                );
            }
        }
    }
}
