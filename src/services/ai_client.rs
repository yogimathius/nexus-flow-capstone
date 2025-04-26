use crate::services::mcp_client::{McpClientTrait, SymbolMeaning};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use std::sync::Arc;
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
    mcp_client: Option<Arc<dyn McpClientTrait>>,
    openrouter_url: String,
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
            mcp_client: None,
            openrouter_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
        }
    }

    // For testing and configuration: set the OpenRouter URL
    pub fn with_openrouter_url(mut self, url: &str) -> Self {
        self.openrouter_url = url.to_string();
        self
    }

    pub fn with_mcp_client<T: McpClientTrait + 'static>(mut self, mcp_client: T) -> Self {
        self.mcp_client = Some(Arc::new(mcp_client));
        self
    }

    pub async fn request_dream_interpretation(
        &self,
        dream_text: &str,
    ) -> Result<String, ClientError> {
        println!(
            "Preparing dream interpretation request for: {:?}",
            dream_text
        );

        // Extract potential symbols from the dream
        let symbols = self.extract_key_symbols(dream_text);

        // Try to look up symbols if MCP client is available
        let symbol_meanings = if let Some(mcp_client) = &self.mcp_client {
            match mcp_client
                .lookup_symbols(symbols.iter().map(|s| s.as_str()).collect())
                .await
            {
                Ok(meanings) => Some(meanings),
                Err(e) => {
                    println!("Failed to look up symbols: {:?}", e);
                    None
                }
            }
        } else {
            None
        };

        // Create a more detailed prompt with instructions and symbol information
        let mut prompt = format!(
            "You are a skilled dream interpreter with expertise in Jungian psychology, symbolism, and dream analysis. 
            
            Please interpret this dream in a thoughtful, insightful way:
            
            \"{}\".",
            dream_text
        );

        // Add symbol information if available
        if let Some(meanings) = symbol_meanings {
            prompt.push_str(
                "\n\nHere are known symbolic meanings that may be relevant to this dream:\n",
            );

            for meaning in meanings {
                prompt.push_str(&format!(
                    "- {} ({}): {}\n",
                    meaning.symbol,
                    meaning.contexts.join(", "),
                    meaning.meaning
                ));
            }
        }

        prompt.push_str("\nProvide analysis of the key symbols, potential meanings, and psychological insights from this dream. Consider archetypes, personal associations, and emotional resonance.");

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

    fn extract_key_symbols(&self, dream_text: &str) -> Vec<String> {
        // Common dream symbols to look for
        let common_symbols = [
            "water", "fire", "flying", "falling", "teeth", "death", "snake", "house", "chase",
            "naked", "test", "school", "maze", "door", "ocean", "mountain", "animal", "bird",
            "child", "parent", "shadow", "light", "darkness", "journey", "vehicle",
        ];

        let text_lower = dream_text.to_lowercase();
        let mut found_symbols = Vec::new();

        for symbol in common_symbols.iter() {
            if text_lower.contains(symbol) {
                found_symbols.push((*symbol).to_string());
            }
        }

        // Find other potential symbols (nouns) - this is a simplified approach
        // In a real implementation, you might want to use NLP for better extraction
        let words: Vec<&str> = dream_text
            .split(|c: char| !c.is_alphanumeric() && c != '\'')
            .filter(|w| !w.is_empty() && w.len() > 3)
            .collect();

        for word in words {
            if !common_symbols.contains(&word.to_lowercase().as_str())
                && !found_symbols.contains(&word.to_lowercase())
            {
                // Basic heuristic: proper nouns often start with capital letters
                if word.chars().next().map_or(false, |c| c.is_uppercase()) {
                    found_symbols.push(word.to_lowercase());
                }
            }
        }

        found_symbols
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
            .post(&self.openrouter_url)
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
