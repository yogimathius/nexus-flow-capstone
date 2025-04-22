use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use std::time::Duration;

// Define error types
#[derive(Debug)]
pub enum ClientError {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
    NoChoicesError,
    EmptyContentError,
    ApiKeyError(String),
    RateLimitedError,
    ModelNotAvailableError,
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::RequestError(e) => write!(f, "Request error: {}", e),
            ClientError::ParseError(e) => write!(f, "Parse error: {}", e),
            ClientError::NoChoicesError => write!(f, "No choices returned in response"),
            ClientError::EmptyContentError => write!(f, "Empty content returned"),
            ClientError::ApiKeyError(e) => write!(f, "API key error: {}", e),
            ClientError::RateLimitedError => write!(f, "Rate limited by API provider"),
            ClientError::ModelNotAvailableError => write!(f, "Model not available"),
        }
    }
}

// OpenRouter request/response types
#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct CompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct ResponseMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct CompletionResponse {
    choices: Vec<Choice>,
}

// Azure AI API request/response types
#[derive(Serialize)]
struct AzureCompletionRequest {
    messages: Vec<Message>,
    max_tokens: u32,
    model: String,
}

#[derive(Deserialize)]
struct AzureChoice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct AzureCompletionResponse {
    choices: Vec<AzureChoice>,
}

// Client implementation
#[derive(Clone)]
pub struct AiClient {
    openrouter_api_key: String,
    github_token: Option<String>,
    client: reqwest::Client,
}

impl AiClient {
    pub fn new(openrouter_api_key: String, github_token: Option<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        AiClient {
            openrouter_api_key,
            github_token,
            client,
        }
    }

    pub async fn request_dream_interpretation(
        &self,
        dream_text: &str,
    ) -> Result<String, ClientError> {
        println!(
            "Preparing dream interpretation request for: {:?}",
            dream_text
        );

        // Create a more detailed prompt with instructions
        let prompt = format!(
            "You are a skilled dream interpreter with expertise in Jungian psychology, symbolism, and dream analysis. 
            
            Please interpret this dream in a thoughtful, insightful way:
            
            \"{}\".
            
            Provide analysis of the key symbols, potential meanings, and psychological insights from this dream. 
            Consider archetypes, personal associations, and emotional resonance.",
            dream_text
        );

        // Try Azure API first if GitHub token is available
        if let Some(github_token) = &self.github_token {
            match self
                .request_azure_interpretation(&prompt, github_token)
                .await
            {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if matches!(
                        e,
                        ClientError::RateLimitedError | ClientError::ModelNotAvailableError
                    ) {
                        println!(
                            "Azure API request failed with limit error, falling back to OpenRouter: {:?}",
                            e
                        );
                        // Fall through to OpenRouter
                    } else {
                        println!("Azure API request failed with error: {:?}", e);
                        return Err(e);
                    }
                }
            }
        }

        // Fallback to OpenRouter API
        self.request_openrouter_interpretation(&prompt).await
    }

    async fn request_azure_interpretation(
        &self,
        prompt: &str,
        github_token: &str,
    ) -> Result<String, ClientError> {
        println!("Sending request to Azure Inference API with DeepSeek-R1 model...");

        let azure_request = AzureCompletionRequest {
            messages: vec![Message {
                role: String::from("user"),
                content: prompt.to_string(),
            }],
            max_tokens: 1000,
            model: String::from("DeepSeek-R1"),
        };

        let response = self
            .client
            .post("https://models.inference.ai.azure.com/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", github_token))
            .json(&azure_request)
            .send()
            .await
            .map_err(ClientError::RequestError)?;

        let status = response.status();
        println!("Received response from Azure API with status: {}", status);

        if status.is_client_error() {
            if status.as_u16() == 429 {
                println!("Azure API rate limit exceeded");
                return Err(ClientError::RateLimitedError);
            } else {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                println!("Azure API client error: {}", error_text);
                return Err(ClientError::ApiKeyError(error_text));
            }
        }

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            println!("Azure API error: {}", error_text);
            return Err(ClientError::ApiKeyError(error_text));
        }

        let response_text = response.text().await.map_err(ClientError::RequestError)?;
        println!("Raw Azure API Response: {}", response_text);

        let azure_completion: AzureCompletionResponse =
            serde_json::from_str(&response_text).map_err(ClientError::ParseError)?;

        match azure_completion.choices.first() {
            Some(choice) => {
                let content = choice.message.content.clone();
                if content.is_empty() {
                    println!("Warning: Received empty content from Azure API");
                    return Err(ClientError::EmptyContentError);
                }
                println!("Successfully extracted interpretation from Azure API response");
                Ok(content)
            }
            None => {
                eprintln!("No choices found in Azure API response");
                Err(ClientError::NoChoicesError)
            }
        }
    }

    async fn request_openrouter_interpretation(&self, prompt: &str) -> Result<String, ClientError> {
        println!("Sending request to OpenRouter API with DeepSeek model...");

        let model = "deepseek/deepseek-r1:free";

        let request_body = CompletionRequest {
            model: String::from(model),
            messages: vec![Message {
                role: String::from("user"),
                content: prompt.to_string(),
            }],
        };

        println!("Using model: {}", request_body.model);

        let response = self
            .client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header(
                "Authorization",
                format!("Bearer {}", self.openrouter_api_key),
            )
            .header("HTTP-Referer", "https://localhost:3000") // Required by OpenRouter
            .header("X-Title", "Dream Interpreter") // Required by OpenRouter
            .json(&request_body)
            .send()
            .await
            .map_err(ClientError::RequestError)?;

        println!(
            "Received response from OpenRouter with status: {}",
            response.status()
        );

        let response_text = response.text().await.map_err(ClientError::RequestError)?;

        println!("Raw API Response: {}", response_text);

        let completion: CompletionResponse =
            serde_json::from_str(&response_text).map_err(ClientError::ParseError)?;

        match completion.choices.first() {
            Some(choice) => {
                let content = choice.message.content.clone();
                if content.is_empty() {
                    println!("Warning: Received empty content from OpenRouter API");
                    return Ok("The AI model couldn't interpret your dream at this time. Please try again later.".to_string());
                }
                println!("Successfully extracted interpretation from OpenRouter API response");
                Ok(content)
            }
            None => {
                eprintln!("No choices found in OpenRouter API response");
                Err(ClientError::NoChoicesError)
            }
        }
    }
}

// Legacy alias for backward compatibility
pub type OpenRouterClient = AiClient;
