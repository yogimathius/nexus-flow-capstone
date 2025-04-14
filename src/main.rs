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

    // Parse command line arguments to get port
    let args: Vec<String> = std::env::args().collect();
    let port = parse_port_from_args(&args).unwrap_or(3000);

    // Check for special run modes
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,rmcp=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("Initializing Model Context Protocol (MCP) Text Processor API...");

    // Create and run the server with the specified port
    let server = Server::new(port);
    println!("Starting server on port {}...", port);
    server.run().await;
}

// Function to parse port from command line arguments
fn parse_port_from_args(args: &[String]) -> Option<u16> {
    for i in 0..args.len() {
        if args[i] == "--port" && i + 1 < args.len() {
            if let Ok(port) = args[i + 1].parse::<u16>() {
                return Some(port);
            }
        }
    }
    None
}
