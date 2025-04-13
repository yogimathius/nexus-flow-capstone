use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct CompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug)]
pub enum ClientError {
    RequestError(ReqwestError),
    ParseError(serde_json::Error),
    NoChoicesError,
}

impl From<ReqwestError> for ClientError {
    fn from(err: ReqwestError) -> Self {
        ClientError::RequestError(err)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        ClientError::ParseError(err)
    }
}

#[derive(Debug)]
pub struct OpenRouterClient {
    api_key: String,
    client: Client,
}

impl OpenRouterClient {
    // Constructor to initialize the client with the API key
    pub fn new(api_key: String) -> Self {
        OpenRouterClient {
            api_key,
            client: Client::new(),
        }
    }

    // Method to make a request to the OpenRouter API and return the interpretation
    pub async fn request_dream_interpretation(
        &self,
        dream_text: &str,
    ) -> Result<String, ClientError> {
        let request_body = CompletionRequest {
            model: String::from("deepseek/deepseek-r1:free"),
            messages: vec![Message {
                role: String::from("user"),
                content: String::from(dream_text),
            }],
        };

        let response = self
            .client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://localhost:3000") // Required by OpenRouter
            .header("X-Title", "Dream Interpreter") // Required by OpenRouter
            .json(&request_body)
            .send()
            .await?;

        // Debug: Print the raw response
        let response_text = response.text().await?;
        println!("Raw API Response: {}", response_text);

        // Parse the response text
        let completion: CompletionResponse = serde_json::from_str(&response_text)?;

        // Extract the first choice's message content
        match completion.choices.first() {
            Some(choice) => Ok(choice.message.content.clone()),
            None => Err(ClientError::NoChoicesError),
        }
    }
}
