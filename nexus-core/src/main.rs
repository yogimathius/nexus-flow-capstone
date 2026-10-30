use axum_server::Server;
use tracing::info;
use std::io;

mod logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize enhanced logging
    logging::setup_logging().expect("Failed to set up logging");

    // Parse command line arguments to get port
    let args: Vec<String> = std::env::args().collect();
    let port = match parse_port_from_args(&args)? {
        Some(port) => port,
        None => match std::env::var("PORT") {
            Ok(value) => value.parse::<u16>().map_err(|error| {
                io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid PORT: {error}"))
            })?,
            Err(std::env::VarError::NotPresent) => 3003,
            Err(error) => return Err(Box::<dyn std::error::Error>::from(error)),
        },
    };

    info!("Initializing Dream Interpreter API...");

    // Create and run the server with the specified port
    let server = Server::new(port);
    info!("Starting server on port {}...", port);
    server.run().await?;

    Ok(())
}

// Function to parse port from command line arguments
fn parse_port_from_args(args: &[String]) -> io::Result<Option<u16>> {
    if let Some(index) = args.iter().position(|arg| arg == "--port") {
        let value = args.get(index + 1).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "--port requires a number")
        })?;
        let port = value.parse::<u16>().map_err(|error| {
            io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid --port: {error}"))
        })?;
        Ok(Some(port))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_port_from_args;

    #[test]
    fn invalid_port_is_reported() {
        assert!(parse_port_from_args(&["api".into(), "--port".into(), "nope".into()]).is_err());
        assert!(parse_port_from_args(&["api".into(), "--port".into()]).is_err());
    }
}
