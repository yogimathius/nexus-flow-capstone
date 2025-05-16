use crate::error::{AppError, AppResult};
use crate::services::mcp_client::McpClientTrait;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;
use std::time::Duration;

// Request/response types
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

// GitHub API request/response types
#[derive(Serialize)]
struct GithubCompletionRequest {
    messages: Vec<Message>,
    max_tokens: u32,
    model: String,
    temperature: f32,
    top_p: f32,
}

#[derive(Deserialize)]
struct GithubChoice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct GithubCompletionResponse {
    choices: Vec<GithubChoice>,
}

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

    // Check if MCP client is initialized
    pub fn has_mcp_client(&self) -> bool {
        self.mcp_client.is_some()
    }

    pub async fn request_dream_interpretation(&self, dream_text: &str) -> AppResult<String> {
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

        // Try GitHub API first if GitHub token is available
        if let Some(token) = &self.github_token {
            match self.request_github_interpretation(&prompt, token).await {
                Ok(interpretation) => return Ok(interpretation),
                Err(e) => {
                    println!("GitHub API error: {:?}, falling back to OpenRouter", e);
                    // Fall back to OpenRouter
                }
            }
        }

        // Use OpenRouter API as fallback
        match self.request_openrouter_interpretation(&prompt).await {
            Ok(interpretation) => Ok(interpretation),
            Err(e) => {
                println!("OpenRouter API error: {:?}", e);
                Err(AppError::ExternalServiceError(format!(
                    "Failed to get interpretation from any API provider: {}",
                    e
                )))
            }
        }
    }

    fn extract_key_symbols(&self, dream_text: &str) -> Vec<String> {
        // Simple symbol extraction based on common keywords
        // This is a basic implementation - in a real system you'd use NLP or ML for better extraction
        let text = dream_text.to_lowercase();
        let common_symbols = [
            "water", "fire", "flying", "falling", "teeth", "death", "snake", "chase", "house",
            "animal", "school", "baby", "car", "moon", "sun", "star", "ocean", "mountain", "door",
            "key", "bridge", "path", "shadow", "light", "mother", "father", "child", "god",
            "devil", "money", "exam", "naked", "wedding", "funeral", "birthday",
        ];

        common_symbols
            .iter()
            .filter(|&symbol| text.contains(symbol))
            .map(|&symbol| symbol.to_string())
            .collect()
    }

    async fn request_github_interpretation(
        &self,
        prompt: &str,
        github_token: &str,
    ) -> AppResult<String> {
        println!("Using GitHub API for interpretation");

        let request_body = GithubCompletionRequest {
            messages: vec![Message {
                role: "user".into(),
                content: prompt.to_string(),
            }],
            max_tokens: 800,
            model: "gpt-4-1106-preview".to_string(),
            temperature: 0.7,
            top_p: 0.9,
        };

        // Try the GitHub API
        let response = self
            .client
            .post("https://api.github.com/copilot_internal/v1/chat_completions")
            .header("Authorization", format!("Bearer {}", github_token))
            .header("Content-Type", "application/json")
            .header("User-Agent", "GitHub InCopilot/1.0")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AppError::HttpError(e))?;

        let status = response.status();
        println!("GitHub API response status: {}", status);

        // Check for rate limiting
        if status.as_u16() == 429 {
            return Err(AppError::RateLimitedError);
        }

        // Check for other errors
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            println!("GitHub API error response: {}", error_text);

            return Err(AppError::ExternalServiceError(format!(
                "GitHub API returned error status: {} - {}",
                status, error_text
            )));
        }

        // Parse the successful response
        let text = response.text().await.map_err(|e| AppError::HttpError(e))?;
        let completion: GithubCompletionResponse =
            serde_json::from_str(&text).map_err(|e| AppError::SerializationError(e))?;

        // Extract the first choice's message content
        match completion.choices.first() {
            Some(choice) if !choice.message.content.is_empty() => {
                Ok(choice.message.content.clone())
            }
            Some(_) => Err(AppError::ApplicationError("Empty content returned".into())),
            None => Err(AppError::ApplicationError(
                "No choices returned in response".into(),
            )),
        }
    }

    async fn request_openrouter_interpretation(&self, prompt: &str) -> AppResult<String> {
        println!("Sending request to OpenRouter API with DeepSeek model...");

        let model = "openai/gpt-4.1";

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
            .await?;

        println!(
            "Received response from OpenRouter with status: {}",
            response.status()
        );

        let response_text = response.text().await?;
        let completion: CompletionResponse = serde_json::from_str(&response_text)?;

        match completion.choices.first() {
            Some(choice) => {
                let content = choice.message.content.clone();
                if content.is_empty() {
                    println!("Warning: Received empty content from OpenRouter API");
                    return Ok("The AI model couldn't interpret your dream at this time. Please try again later.".to_string());
                }
                Ok(content)
            }
            None => Err(AppError::ApplicationError(
                "No choices returned in response".into(),
            )),
        }
    }
}
