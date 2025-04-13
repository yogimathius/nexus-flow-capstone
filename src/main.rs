use axum_server::create_app;
use dotenv::dotenv;
use std::env;
use tracing_subscriber::{self, prelude::*};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Setup more detailed logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,rmcp=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("Initializing Model Context Protocol (MCP) Text Processor API...");

    // Create the app using our library function
    let app = create_app();

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started on port 3000");
    println!("Available endpoints:");
    println!("  GET  /health - Health check endpoint");
    println!("  POST /api/process_text - Process text with operations");
    println!("");
    println!("Example usage:");
    println!("  curl -X POST http://localhost:3000/api/process_text \\");
    println!("    -H \"Content-Type: application/json\" \\");
    println!(
        "    -d '{{\"text\": \"this is a sample text. it needs formatting!\", \"operation\": \"format\"}}'"
    );
    println!("");
    println!("Available operations:");
    println!("  count    - Count characters, words, and lines");
    println!("  validate - Check text for validity and patterns");
    println!("  format   - Format and clean text (capitalize, etc.)");
    println!("  analyze  - Analyze text sentiment");

    axum::serve(listener, app).await.unwrap();
}
