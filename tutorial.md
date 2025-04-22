# Building a Dream Interpreter API with Axum and MCP

This tutorial guides you through creating a REST API that interprets dreams using the Model Context Protocol (MCP) and an AI client (OpenRouter). It provides a structured approach using proper server implementation.

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

### Step 2: Add dependencies

```bash
# Add core dependencies
cargo add axum
cargo add tokio --features full
cargo add serde --features derive
cargo add serde_json
cargo add tracing
cargo add tracing-subscriber --features env-filter
cargo add dotenv

# Add MCP dependencies
cargo add rmcp --features "server client transport-child-process"

# Add HTTP client for OpenRouter API
cargo add reqwest --features json
```

### Step 3: Set up project structure

Create the following files and directories:

```
src/
├── main.rs            # Entry point
├── lib.rs             # Module exports
├── mcp_client.rs      # MCP client implementation
├── ai_client.rs       # OpenRouter AI client
└── server.rs          # Server implementation
```

## Implementation

### Step 4: Create the AI Client (src/ai_client.rs)

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
        dream_text: &str,
    ) -> Result<String, ClientError> {
        let request_body = CompletionRequest {
            model: String::from("deepseek/deepseek-r1:free"),
            messages: vec![Message {
                role: String::from("user"),
                content: format!("Please interpret this dream: {}", dream_text),
            }],
        };

        println!("Sending request to OpenRouter API...");
        println!("Using model: {}", request_body.model);

        let response = self
            .client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://localhost:3000") // Required by OpenRouter
            .header("X-Title", "Dream Interpreter") // Required by OpenRouter
            .json(&request_body)
            .send()
            .await?;

        // Debug: Print the raw response status
        println!("Received response from OpenRouter with status: {}", response.status());

        // Get the response text
        let response_text = response.text().await?;
        println!("Raw API Response length: {} bytes", response_text.len());

        // Parse the response text
        let completion: CompletionResponse = serde_json::from_str(&response_text)?;
        println!("Successfully parsed JSON response");

        // Extract the first choice's message content
        match completion.choices.first() {
            Some(choice) => Ok(choice.message.content.clone()),
            None => Err(ClientError::NoChoicesError),
        }
    }
}
```

### Step 5: Create the MCP client (src/mcp_client.rs)

```rust
use rmcp::{ServiceExt, object, transport::TokioChildProcess};
use serde_json;
use std::error::Error;
use tokio::process::Command;
use tracing::{debug, error, info};

pub struct McpClient {
    client: rmcp::service::RunningService<rmcp::RoleClient, ()>,
}

impl McpClient {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        info!("Initializing MCP client...");

        // Initialize the MCP client with the server-everything implementation
        let mut command = Command::new("npx");
        command.arg("-y")
               .arg("@modelcontextprotocol/server-everything");

        debug!("Starting MCP server with command: {:?}", command);

        let client = match ().serve(TokioChildProcess::new(&mut command)?).await {
            Ok(client) => {
                info!("MCP client successfully initialized");
                client
            }
            Err(e) => {
                error!("Failed to initialize MCP client: {:?}", e);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to initialize MCP client: {}", e),
                )));
            }
        };

        Ok(Self { client })
    }

    // Process text using the specified operation
    pub async fn process_text(
        &self,
        text: &str,
        operation: TextOperation,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        info!("Processing text with operation: {:?}", operation);

        // First attempt to use MCP if available
        match self.try_mcp_processing(text, &operation).await {
            Ok(result) => {
                info!("Successfully processed text through MCP");
                return Ok(result);
            }
            Err(e) => {
                info!("MCP processing unavailable: {}. Using local processing.", e);
                // Fall back to local processing
                self.process_locally(text, operation)
            }
        }
    }

    // Try to process text using MCP
    async fn try_mcp_processing(
        &self,
        text: &str,
        operation: &TextOperation,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Try to use MCP for processing
        let op_name = match operation {
            TextOperation::Count => "text_count",
            TextOperation::Validate => "text_validate",
            TextOperation::Format => "text_format",
            TextOperation::Analyze => "text_analyze",
        };

        debug!("Attempting to use MCP tool: {}", op_name);

        match self
            .client
            .call_tool(rmcp::model::CallToolRequestParam {
                name: op_name.into(),
                arguments: Some(object!({
                    "text": text
                })),
            })
            .await
        {
            Ok(result) => {
                let json_result = serde_json::to_value(&result)?;
                if let Some(result_str) = json_result.as_str() {
                    return Ok(result_str.to_string());
                }

                if let Some(result_obj) = json_result.as_object() {
                    return Ok(serde_json::to_string_pretty(result_obj)?);
                }

                Ok(json_result.to_string())
            }
            Err(e) => {
                debug!("MCP tool unavailable: {:?}", e);
                Err(format!("MCP tool '{}' not available", op_name).into())
            }
        }
    }

    // Fallback local processing
    fn process_locally(
        &self,
        text: &str,
        operation: TextOperation,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        info!("Processing text locally with operation: {:?}", operation);

        match operation {
            TextOperation::Count => {
                let char_count = text.chars().count();
                let word_count = text.split_whitespace().count();
                let line_count = text.lines().count();

                Ok(format!(
                    "Text statistics:\n- Characters: {}\n- Words: {}\n- Lines: {}",
                    char_count, word_count, line_count
                ))
            }
            TextOperation::Validate => {
                // Simple validation logic
                let is_empty = text.trim().is_empty();
                let has_numbers = text.chars().any(|c| c.is_numeric());
                let has_special = text
                    .chars()
                    .any(|c| !c.is_alphanumeric() && !c.is_whitespace());

                let mut validations = Vec::new();
                if is_empty {
                    validations.push("Text is empty or whitespace only");
                }
                if has_numbers {
                    validations.push("Contains numeric characters");
                }
                if has_special {
                    validations.push("Contains special characters");
                }

                let result = if validations.is_empty() {
                    "Text is valid plain text (alphabetic characters only)".to_string()
                } else {
                    format!("Validation results:\n- {}", validations.join("\n- "))
                };

                Ok(result)
            }
            // Other operations implementation...
            TextOperation::Format => {
                // Implementation for formatting text
                Ok("Formatted text".to_string())
            }
            TextOperation::Analyze => {
                // Implementation for analyzing text
                Ok("Text analysis".to_string())
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextOperation {
    Count,    // Count characters, words, etc.
    Validate, // Check if text matches certain criteria
    Format,   // Format or clean text
    Analyze,  // Analyze sentiment or content
}

impl TextOperation {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "count" => Ok(Self::Count),
            "validate" => Ok(Self::Validate),
            "format" => Ok(Self::Format),
            "analyze" => Ok(Self::Analyze),
            _ => Err(format!(
                "Unknown operation: {}. Supported operations: count, validate, format, analyze",
                s
            )),
        }
    }
}
```

### Step 6: Create the Server Implementation (src/server.rs)

```rust
use axum::{
    Router,
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;

use crate::ai_client::OpenRouterClient;
use crate::mcp_client::{McpClient, TextOperation};

// API Types
#[derive(Deserialize)]
pub struct TextRequest {
    pub text: String,
    pub operation: String,
}

#[derive(Serialize)]
pub struct TextResponse {
    pub result: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct DreamRequest {
    pub dream_text: String,
}

#[derive(Serialize)]
pub struct DreamResponse {
    pub interpretation: String,
    pub status: String,
}

// Server implementation
pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }

    pub fn router(&self) -> Router {
        Router::new()
            .route("/health", get(health_handler))
            .route("/api/process_text", post(process_text_handler))
            .route("/api/interpret_dream", post(interpret_dream_handler))
    }

    pub async fn run(&self) {
        let app = self.router();

        // Build the server address
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        println!("Server listening on {}", addr);

        // Print available endpoints
        self.print_available_endpoints();

        // Start the server
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap_or_else(|e| {
            eprintln!("Server error: {:?}", e);
        });
    }

    fn print_available_endpoints(&self) {
        println!("Available endpoints:");
        println!("  GET  /health - Health check endpoint");
        println!("  POST /api/process_text - Process text with operations");
        println!("  POST /api/interpret_dream - Interpret dreams using AI");
        println!("");
        println!("Example usage:");
        println!("  curl -X POST http://localhost:{}/api/process_text \\", self.port);
        println!("    -H \"Content-Type: application/json\" \\");
        println!("    -d '{{\"text\": \"this is a sample text. it needs formatting!\", \"operation\": \"format\"}}'");
        println!("");
        println!("Dream interpretation example:");
        println!("  curl -v -X POST http://localhost:{}/api/interpret_dream \\", self.port);
        println!("    -H \"Content-Type: application/json\" \\");
        println!("    -d '{{\"dream_text\": \"I was flying over a city made of crystal.\"}}'");
        println!("");
    }
}

// Handler functions
async fn health_handler() -> &'static str {
    "ok"
}

async fn process_text_handler(Json(payload): Json<TextRequest>) -> impl IntoResponse {
    println!("Process text handler called with operation: {}", payload.operation);

    // Parse the operation
    let operation = match TextOperation::from_str(&payload.operation) {
        Ok(op) => op,
        Err(e) => {
            return Json(TextResponse {
                result: e,
                status: "error".to_string(),
            });
        }
    };

    // Initialize MCP client
    match McpClient::new().await {
        Ok(client) => match client.process_text(&payload.text, operation).await {
            Ok(result) => Json(TextResponse {
                result,
                status: "success".to_string(),
            }),
            Err(e) => {
                eprintln!("Error processing text: {:?}", e);
                Json(TextResponse {
                    result: format!("Could not process text: {}", e),
                    status: "error".to_string(),
                })
            }
        },
        Err(e) => {
            eprintln!("Failed to initialize MCP client: {:?}", e);
            Json(TextResponse {
                result: "Failed to initialize text processor".to_string(),
                status: "error".to_string(),
            })
        }
    }
}

async fn interpret_dream_handler(Json(payload): Json<DreamRequest>) -> impl IntoResponse {
    println!("Dream handler called with text: {}", payload.dream_text);

    // Initialize AI client
    let api_key = match std::env::var("OPENROUTER_API_KEY") {
        Ok(key) => {
            println!("Successfully loaded API key with length: {}", key.len());
            key
        }
        Err(e) => {
            eprintln!("Failed to load API key: {:?}", e);
            return Json(DreamResponse {
                interpretation: format!("Error: Could not load API key: {:?}", e),
                status: "error".to_string(),
            });
        }
    };

    println!("Initializing AI client with API key");
    let ai_client = OpenRouterClient::new(api_key);

    // Request dream interpretation
    println!("Sending dream text to OpenRouter...");
    match ai_client
        .request_dream_interpretation(&payload.dream_text)
        .await
    {
        Ok(interpretation) => {
            println!("Successfully received interpretation");
            Json(DreamResponse {
                interpretation,
                status: "success".to_string(),
            })
        }
        Err(e) => {
            eprintln!("Error interpreting dream: {:?}", e);
            Json(DreamResponse {
                interpretation: format!("Could not interpret dream: {:?}", e),
                status: "error".to_string(),
            })
        }
    }
}
```

### Step 7: Module Exports (src/lib.rs)

```rust
pub mod ai_client;
pub mod mcp_client;
pub mod server;

// Re-export the Server struct for easy access
pub use server::Server;
```

### Step 8: Create the Entry Point (src/main.rs)

```rust
use axum_server::Server;
use dotenv::dotenv;
use std::env;
use tracing_subscriber::{self, prelude::*};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Debug print to check if the API key was loaded
    match env::var("OPENROUTER_API_KEY") {
        Ok(key) => println!("API key found with length: {}", key.len()),
        Err(e) => println!("Error loading API key: {:?}", e),
    }

    // Setup more detailed logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,rmcp=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("Initializing Model Context Protocol (MCP) Text Processor API...");

    // Create and run the server
    let server = Server::new(3003);
    server.run().await;
}
```

### Step 9: Create a .env file

Create a `.env` file in the root of your project:

```
OPENROUTER_API_KEY=your_api_key_here
```

Replace `your_api_key_here` with your actual OpenRouter API key.

## Running the Application

1. Make sure you have Node.js and npm installed (required for MCP).

2. Create the .env file with your OpenRouter API key.

3. Build and run the application:

   ```bash
   cargo run
   ```

4. The server will start on port 3000, showing available endpoints.

5. Test the text processing endpoint:

   ```bash
   curl -X POST http://localhost:3000/api/process_text \
     -H "Content-Type: application/json" \
     -d '{"text": "this is a sample text. it needs formatting!", "operation": "format"}'
   ```

6. Test the dream interpretation endpoint:
   ```bash
   curl -v -X POST http://localhost:3000/api/interpret_dream \
     -H "Content-Type: application/json" \
     -d '{"dream_text": "I was flying over a city made of crystal."}'
   ```

## How It Works

1. **Server Structure**: The application uses a structured Server implementation with clear separation of concerns.

2. **MCP Client**: For text processing operations, the application uses the Model Context Protocol to interact with AI models.

3. **OpenRouter Client**: For dream interpretation, the application uses the OpenRouter API to access powerful language models.

4. **Environment Variables**: API keys are stored in the .env file and loaded at runtime.

5. **Axum Web Framework**: The application uses the Axum web framework for handling HTTP requests and routing.

## Next Steps

Here are some ways to extend this project:

1. **Add authentication**: Implement API keys for access control.

2. **Add a database**: Store dream interpretations and user data.

3. **Create a web frontend**: Build a simple web interface for interacting with the API.

4. **Add more AI capabilities**: Implement more advanced text analysis features.
