use axum_server::Server;
use tracing::info;

mod logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize enhanced logging
    logging::setup_logging().expect("Failed to set up logging");

    // Parse command line arguments to get port
    let args: Vec<String> = std::env::args().collect();
    let port = parse_port_from_args(&args).unwrap_or(3003);

    info!("Initializing Dream Interpreter API...");

    // Create and run the server with the specified port
    let server = Server::new(port);
    info!("Starting server on port {}...", port);
    server.run().await;

    Ok(())
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
