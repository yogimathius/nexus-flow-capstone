use std::env;
use std::fmt;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub openrouter_api_key: String,
    pub github_token: Option<String>,
    pub port: u16,
    pub mcp_server_url: Option<String>,
    pub enable_mcp: bool,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(String),
    ParseError(String),
    DotEnvError(dotenvy::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingEnv(var) => write!(f, "Missing environment variable: {}", var),
            ConfigError::ParseError(err) => write!(f, "Error parsing config: {}", err),
            ConfigError::DotEnvError(err) => write!(f, "Error loading .env file: {}", err),
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // Ensure .env file is loaded
        let dot_env_result = dotenvy::dotenv();
        if let Err(e) = dot_env_result {
            println!("Warning: .env file could not be loaded: {}", e);
            // Continue anyway, in case env vars are set another way
        } else {
            println!("Loaded environment from .env file successfully");
        }

        // Required environment variables
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnv("DATABASE_URL".to_string()))?;

        let openrouter_api_key = env::var("OPENROUTER_API_KEY")
            .map_err(|_| ConfigError::MissingEnv("OPENROUTER_API_KEY".to_string()))?;

        // Optional environment variables
        let github_token = match env::var("GITHUB_TOKEN") {
            Ok(token) => {
                println!("GitHub token loaded successfully, length: {}", token);
                Some(token)
            }
            Err(_) => {
                println!("No GitHub token found in environment variables");
                None
            }
        };

        // Optional port with default value
        let port = match env::var("PORT") {
            Ok(port_str) => port_str
                .parse::<u16>()
                .map_err(|e| ConfigError::ParseError(format!("Invalid PORT: {}", e)))?,
            Err(_) => 3003, // Default port
        };

        // Optional MCP configuration
        let mcp_server_url = match env::var("MCP_SERVER_URL") {
            Ok(url) => {
                println!("MCP server URL configured: {}", url);
                Some(url)
            }
            Err(_) => {
                println!("No MCP server URL specified, will use default (127.0.0.1:3002)");
                None
            }
        };

        let enable_mcp = match env::var("ENABLE_MCP") {
            Ok(val) => match val.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => {
                    println!("MCP client is enabled");
                    true
                }
                _ => {
                    println!("MCP client is disabled by configuration");
                    false
                }
            },
            Err(_) => {
                println!("ENABLE_MCP not specified, defaulting to enabled");
                true
            }
        };

        Ok(Config {
            database_url,
            openrouter_api_key,
            github_token,
            port,
            mcp_server_url,
            enable_mcp,
        })
    }
}
