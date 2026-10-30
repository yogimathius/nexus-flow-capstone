use std::{env, fmt};

pub struct Config {
    pub database_url: String,
    pub openrouter_api_key: String,
    pub github_token: Option<String>,
    pub enable_mcp: bool,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(&'static str),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEnv(name) => write!(f, "Missing environment variable: {name}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // An environment variable takes precedence over a local .env file.
        let _ = dotenvy::dotenv();

        let database_url = env::var("DATABASE_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .ok_or(ConfigError::MissingEnv("DATABASE_URL"))?;
        let openrouter_api_key = env::var("OPENROUTER_API_KEY").unwrap_or_default();
        let github_token = env::var("GITHUB_TOKEN")
            .ok()
            .filter(|token| !token.trim().is_empty());
        let enable_mcp = env::var("ENABLE_MCP")
            .map(|value| matches!(value.to_ascii_lowercase().as_str(), "true" | "1" | "yes" | "on"))
            .unwrap_or(false);

        Ok(Self {
            database_url,
            openrouter_api_key,
            github_token,
            enable_mcp,
        })
    }
}
