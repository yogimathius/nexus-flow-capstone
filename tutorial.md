# Building a Dream Interpreter API with Axum, AI, and MCP

This tutorial guides you through building a REST API using Axum that interprets dreams using AI models via both OpenRouter and the Model Context Protocol (MCP).

## Prerequisites

- Rust and Cargo installed
- Node.js and npm installed (for MCP server)
- Basic familiarity with Rust and async programming

## Project Setup

### Step 1: Create a new Rust project

```bash
cargo new --bin axum-server
cd axum-server
```

### Step 2: Add dependencies using cargo add

```bash
# Add core dependencies
cargo add axum
cargo add tokio --features full
cargo add serde --features derive
cargo add serde_json

# Add API and networking dependencies
cargo add reqwest --features json
cargo add rmcp --features "server client transport-child-process"

# Add logging dependencies
cargo add tracing
cargo add tracing-subscriber

# Add other helpful dependencies
cargo add chrono --features serde
```

### Step 3: Set up project structure

Create the following files:

- `src/main.rs` - Entry point
- `src/lib.rs` - Main application logic
- `src/ai_client.rs` - OpenRouter API client
- `src/mcp_client.rs` - Model Context Protocol client

## Building the Application Components

### Step 4: Implement the main entry point (src/main.rs)

```rust
use axum_server::create_app;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Setup logging
    tracing_subscriber::fmt::init();

    // Create the app using our library function
    let app = create_app();

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started on port 3000");
    axum::serve(listener, app).await.unwrap();
}
```

### Step 5: Set up the OpenRouter AI client (src/ai_client.rs)

```rust
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
        dream_description: &str,
    ) -> Result<String, ClientError> {
        let request_body = CompletionRequest {
            model: String::from("deepseek/deepseek-r1:free"),
            messages: vec![Message {
                role: String::from("user"),
                content: String::from(dream_description),
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

        let response_text = response.text().await?;

        // Parse the response text
        let completion: CompletionResponse = serde_json::from_str(&response_text)?;

        // Extract the first choice's message content
        match completion.choices.first() {
            Some(choice) => Ok(choice.message.content.clone()),
            None => Err(ClientError::NoChoicesError),
        }
    }
}
```

### Step 6: Create the MCP client (src/mcp_client.rs)

```rust
use rmcp::{ClientHandler, ServiceExt, transport::TokioChildProcess};
use std::error::Error;
use tokio::process::Command;

pub struct McpClient {
    client: rmcp::service::RunningService<rmcp::RoleClient, ()>,
}

impl McpClient {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // Initialize the MCP client with the server-everything implementation
        let client: rmcp::service::RunningService<rmcp::RoleClient, ()> = ()
            .serve(TokioChildProcess::new(
                Command::new("npx")
                    .arg("-y")
                    .arg("@modelcontextprotocol/server-everything"),
            )?)
            .await?;

        Ok(Self { client })
    }

    pub async fn interpret_dream(&self, dream_text: &str) -> Result<String, Box<dyn Error>> {
        // Here we'll implement the dream interpretation using MCP tools
        // For now, returning a placeholder
        Ok("Dream interpretation will be implemented using MCP".to_string())
    }
}
```

### Step 7: Set up the application logic (src/lib.rs)

```rust
pub mod ai_client;
pub mod mcp_client;

use axum::{
    Router,
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::env;

use ai_client::OpenRouterClient;

// API Types
#[derive(Deserialize)]
pub struct DreamRequest {
    pub dream_text: String,
}

#[derive(Serialize)]
pub struct DreamResponse {
    pub interpretation: String,
    pub status: String,
}

// Create a function to build and return the Router (for testing)
pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/interpret_dream", post(interpret_dream))
}

// Handler functions
pub async fn health_check() -> impl IntoResponse {
    "ok"
}

pub async fn interpret_dream(Json(payload): Json<DreamRequest>) -> Json<DreamResponse> {
    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");
    let client = OpenRouterClient::new(api_key);

    match client
        .request_dream_interpretation(&payload.dream_text)
        .await
    {
        Ok(interpretation) => Json(DreamResponse {
            interpretation,
            status: "success".to_string(),
        }),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            Json(DreamResponse {
                interpretation: "Could not process dream interpretation".to_string(),
                status: "error".to_string(),
            })
        }
    }
}
```

## Next Steps for Full MCP Integration

1. **Complete the MCP Client Implementation**:

   ```rust
   pub async fn interpret_dream(&self, dream_text: &str) -> Result<String, Box<dyn Error>> {
       // Create prompt for dream interpretation
       let prompt = format!("Please interpret this dream:\n\n{}", dream_text);

       // Get handlers for interaction with MCP
       let chat = self.client.get_handler::<rmcp::core::Chat>().await?;

       // Send the chat completion request
       let response = chat.completion(
           rmcp::core::ModelRequest {
               model: "gpt-3.5-turbo".to_string(),
               max_tokens: Some(500),
               temperature: Some(0.7),
               ..Default::default()
           },
           vec![rmcp::core::ChatMessage {
               role: "user".to_string(),
               content: prompt,
               ..Default::default()
           }]
       ).await?;

       // Return the interpreted dream
       Ok(response.choices[0].message.content.clone())
   }
   ```

2. **Create a Service Factory**:

   ```rust
   // In lib.rs
   // Add a factory function to choose between AI clients
   async fn get_dream_interpreter(provider: &str) -> Box<dyn DreamInterpreter> {
       match provider {
           "openrouter" => {
               let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");
               Box::new(OpenRouterClient::new(api_key))
           },
           "mcp" => {
               match McpClient::new().await {
                   Ok(client) => Box::new(client),
                   Err(_) => {
                       eprintln!("Failed to initialize MCP client, falling back to OpenRouter");
                       let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");
                       Box::new(OpenRouterClient::new(api_key))
                   }
               }
           },
           _ => {
               eprintln!("Unknown provider {}, falling back to OpenRouter", provider);
               let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");
               Box::new(OpenRouterClient::new(api_key))
           }
       }
   }
   ```

3. **Create a Common Interface**:

   ```rust
   // In a new file src/interpreter.rs
   use async_trait::async_trait;

   #[async_trait]
   pub trait DreamInterpreter {
       async fn interpret_dream(&self, dream_text: &str) -> Result<String, Box<dyn std::error::Error>>;
   }

   // Then implement for both client types
   #[async_trait]
   impl DreamInterpreter for OpenRouterClient {
       async fn interpret_dream(&self, dream_text: &str) -> Result<String, Box<dyn std::error::Error>> {
           self.request_dream_interpretation(dream_text)
               .await
               .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
       }
   }

   #[async_trait]
   impl DreamInterpreter for McpClient {
       async fn interpret_dream(&self, dream_text: &str) -> Result<String, Box<dyn std::error::Error>> {
           self.interpret_dream(dream_text).await
       }
   }
   ```

4. **Update API to Support Provider Selection**:

   ```rust
   #[derive(Deserialize)]
   pub struct DreamRequest {
       pub dream_text: String,
       pub provider: Option<String>, // "openrouter" or "mcp"
   }

   pub async fn interpret_dream(Json(payload): Json<DreamRequest>) -> Json<DreamResponse> {
       let provider = payload.provider.unwrap_or_else(|| "openrouter".to_string());
       let interpreter = get_dream_interpreter(&provider).await;

       match interpreter.interpret_dream(&payload.dream_text).await {
           Ok(interpretation) => Json(DreamResponse {
               interpretation,
               status: "success".to_string(),
           }),
           Err(e) => {
               eprintln!("Error: {:?}", e);
               Json(DreamResponse {
                   interpretation: "Could not process dream interpretation".to_string(),
                   status: "error".to_string(),
               })
           }
       }
   }
   ```

5. **Add Error Handling and Logging**:

   - Implement proper error types
   - Add detailed logging for debugging
   - Add middleware for request tracking

6. **Add Configuration**:

   - Use a config file or environment variables
   - Allow configuration of models, API endpoints, etc.

7. **Add Tests**:
   - Unit tests for each component
   - Integration tests for the API endpoints
   - Mocks for the AI clients to test without calling external services
