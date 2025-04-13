# Building a Dream Interpreter API with Axum and MCP

This tutorial guides you through creating a REST API that interprets dreams using the Model Context Protocol (MCP). MCP allows you to interact with AI models locally, providing a standardized interface across different models.

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

# Add MCP dependencies
cargo add rmcp --features "server client transport-child-process"
```

### Step 3: Set up project structure

Create the following files:

- `src/main.rs` - Entry point
- `src/lib.rs` - Main application logic
- `src/mcp_client.rs` - Model Context Protocol client

## Implementation

### Step 4: Create the MCP client (src/mcp_client.rs)

```rust
use rmcp::{ServiceExt, object, transport::TokioChildProcess};
use serde_json;
use std::error::Error;
use tokio::process::Command;

pub struct McpClient {
    client: rmcp::service::RunningService<rmcp::RoleClient, ()>,
}

impl McpClient {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        // Initialize the MCP client with the server-everything implementation
        let client = ()
            .serve(TokioChildProcess::new(
                Command::new("npx")
                    .arg("-y")
                    .arg("@modelcontextprotocol/server-everything"),
            )?)
            .await?;

        Ok(Self { client })
    }

    pub async fn interpret_dream(
        &self,
        dream_text: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Create the prompt
        let prompt = format!("Please interpret this dream:\n\n{}", dream_text);

        // Use the call_tool method to interact with a completion tool
        let tool_result = self
            .client
            .call_tool(rmcp::model::CallToolRequestParam {
                name: "completion".into(),
                arguments: Some(object!({
                    "model": "gpt-3.5-turbo",
                    "temperature": 0.7,
                    "max_tokens": 500,
                    "messages": [{
                        "role": "user",
                        "content": prompt
                    }]
                })),
            })
            .await?;

        // Convert the tool result to a JSON value we can work with
        let json_result = serde_json::to_value(&tool_result)?;

        // Try to extract the response content
        if let Some(choices) = json_result.get("choices") {
            if let Some(first_choice) = choices.get(0) {
                if let Some(message) = first_choice.get("message") {
                    if let Some(content) = message.get("content") {
                        if let Some(text) = content.as_str() {
                            return Ok(text.to_string());
                        }
                    }
                }
            }
        }

        // Try another common structure pattern
        if let Some(output) = json_result.get("output") {
            if let Some(content) = output.get("content") {
                if let Some(text) = content.as_str() {
                    return Ok(text.to_string());
                }
            }
        }

        // If we couldn't extract the content through the expected paths
        Err("Could not extract content from response".into())
    }
}
```

### Step 5: Set up the application logic (src/lib.rs)

```rust
pub mod mcp_client;
pub mod ai_client;

use axum::{
    Router,
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::env;

use mcp_client::McpClient;
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

// Create a function to build and return the Router
pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route(
            "/api/interpret_dream",
            post(|Json(payload): Json<DreamRequest>| async move {
                // Initialize AI client
                let api_key = env::var("OPENROUTER_API_KEY").expect("API key not set");
                let ai_client = OpenRouterClient::new(api_key);

                // Request dream interpretation
                match ai_client.request_dream_interpretation(&payload.dream_text).await {
                    Ok(interpretation) => Json(DreamResponse {
                        interpretation,
                        status: "success".to_string(),
                    }),
                    Err(e) => {
                        eprintln!("Error interpreting dream: {:?}", e);
                        Json(DreamResponse {
                            interpretation: format!("Could not interpret dream: {:?}", e),
                            status: "error".to_string(),
                        })
                    }
                }
            }),
        )
}
```

### Step 6: Create the entry point (src/main.rs)

```rust
use axum_server::create_app;
use tracing_subscriber::{self, prelude::*};

#[tokio::main]
async fn main() {
    // Setup logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info,rmcp=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("Initializing Model Context Protocol (MCP) Text Processor API...");

    // Create the app
    let app = create_app();

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started on port 3000");
    println!("Available endpoints:");
    println!("  GET  /health - Health check endpoint");
    println!("  POST /api/process_text - Process text with operations");
    println!("  POST /api/interpret_dream - Interpret dreams using AI client");
    println!("");
    println!("Example usage:");
    println!("  curl -X POST http://localhost:3000/api/interpret_dream \\");
    println!("    -H \"Content-Type: application/json\" \\");
    println!("    -d '{\"dream_text\": \"I was flying over a city made of crystal.\"}'");

    axum::serve(listener, app).await.unwrap();
}
```

## Running the Application

1. Start the server:

```bash
cargo run
```

2. Send a request to interpret a dream:

```bash
curl -X POST http://localhost:3000/api/interpret_dream \
  -H "Content-Type: application/json" \
  -d '{"dream_text": "I was flying over mountains made of glass while a golden bird guided me through clouds."}'
```

## How It Works

1. When a request is received, the server initializes an MCP client, which launches the MCP server via npx
2. The dream text is formatted into a prompt
3. The client sends the prompt to the MCP server using the `call_tool` method
4. The MCP server processes the request and returns an interpretation
5. The server parses the response and returns it to the client

## Next Steps

Here are some ways to extend this project:

1. **Add configuration options**:

   - Allow users to specify the model to use
   - Configure temperature and other parameters

2. **Improve error handling**:

   - Add more detailed error logging
   - Implement retries for failed requests

3. **Add authentication**:

   - Implement API keys for access control
   - Add rate limiting

4. **Expand API capabilities**:

   - Add endpoints for different types of dream interpretations
   - Implement a streaming response for longer interpretations

5. **Add a frontend**:

   - Create a simple web interface for submitting dreams
   - Display interpretations in a user-friendly format

6. **Implement caching**:
   - Cache responses to improve performance
   - Implement a database to store dream interpretations
