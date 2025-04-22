use std::env;
use std::fmt;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub openrouter_api_key: String,
    pub github_token: Option<String>,
    pub port: u16,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(String),
    ParseError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingEnv(var) => write!(f, "Missing environment variable: {}", var),
            ConfigError::ParseError(err) => write!(f, "Error parsing config: {}", err),
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // Required environment variables
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnv("DATABASE_URL".to_string()))?;

        let openrouter_api_key = env::var("OPENROUTER_API_KEY")
            .map_err(|_| ConfigError::MissingEnv("OPENROUTER_API_KEY".to_string()))?;

        // Optional environment variables
        let github_token = env::var("GITHUB_TOKEN").ok();

        // Optional port with default value
        let port = match env::var("PORT") {
            Ok(port_str) => port_str
                .parse::<u16>()
                .map_err(|e| ConfigError::ParseError(format!("Invalid PORT: {}", e)))?,
            Err(_) => 3003, // Default port
        };

        Ok(Config {
            database_url,
            openrouter_api_key,
            github_token,
            port,
        })
    }
}
