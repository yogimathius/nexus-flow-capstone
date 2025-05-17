use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    // Database errors
    DatabaseError(sqlx::Error),
    // API/HTTP errors
    HttpError(reqwest::Error),
    // JSON serialization/deserialization errors
    SerializationError(serde_json::Error),
    // MCP related errors
    McpError(String),
    // API key related errors
    ApiKeyError(String),
    // General application errors
    ApplicationError(String),
    // Not found error
    NotFoundError(String),
    // Rate limiting errors
    RateLimitedError,
    // External service errors
    ExternalServiceError(String),
    // Authorization error
    AuthorizationError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::HttpError(e) => write!(f, "HTTP error: {}", e),
            AppError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            AppError::McpError(msg) => write!(f, "MCP error: {}", msg),
            AppError::ApiKeyError(msg) => write!(f, "API key error: {}", msg),
            AppError::ApplicationError(msg) => write!(f, "Application error: {}", msg),
            AppError::NotFoundError(msg) => write!(f, "Not found: {}", msg),
            AppError::RateLimitedError => write!(f, "Rate limited by API provider"),
            AppError::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            AppError::AuthorizationError => write!(f, "Authorization error"),
        }
    }
}

impl Error for AppError {}

// Implement From traits for common error types
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::HttpError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err)
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::ApplicationError(err)
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        AppError::ApplicationError(err.to_string())
    }
}

// Utility function to map any error type to AppError
pub fn map_err<E: std::fmt::Display>(err: E) -> AppError {
    AppError::ApplicationError(format!("{}", err))
}

// Result type alias using AppError
pub type AppResult<T> = Result<T, AppError>;
