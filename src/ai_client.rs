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

        // Use the deepseek model as requested
        let model = "deepseek/deepseek-r1:free";

        let request_body = CompletionRequest {
            model: String::from(model),
            messages: vec![Message {
                role: String::from("user"),
                content: prompt,
            }],
        };

        println!("Sending request to OpenRouter API...");
        println!("Using model: {}", request_body.model);

        let response = match self
            .client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://localhost:3000") // Required by OpenRouter
            .header("X-Title", "Dream Interpreter") // Required by OpenRouter
            .json(&request_body)
            .send()
            .await
        {
            Ok(resp) => {
                println!(
                    "Received response from OpenRouter with status: {}",
                    resp.status()
                );
                resp
            }
            Err(e) => {
                eprintln!("Error sending request to OpenRouter: {:?}", e);
                return Err(ClientError::RequestError(e));
            }
        };

        // Debug: Print the raw response
        let response_text = match response.text().await {
            Ok(text) => {
                println!("Raw API Response length: {} bytes", text.len());
                text
            }
            Err(e) => {
                eprintln!("Error reading response text: {:?}", e);
                return Err(ClientError::RequestError(e));
            }
        };

        println!("Raw API Response: {}", response_text);

        // Parse the response text
        let completion: CompletionResponse = match serde_json::from_str(&response_text) {
            Ok(comp) => {
                println!("Successfully parsed JSON response");
                comp
            }
            Err(e) => {
                eprintln!("Error parsing JSON response: {:?}", e);
                return Err(ClientError::ParseError(e));
            }
        };

        // Extract the first choice's message content
        match completion.choices.first() {
            Some(choice) => {
                println!("Successfully extracted interpretation from response");
                Ok(choice.message.content.clone())
            }
            None => {
                eprintln!("No choices found in response");
                Err(ClientError::NoChoicesError)
            }
        }
    }
}
