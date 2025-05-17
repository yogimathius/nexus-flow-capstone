use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationResponse {
    pub status: String,
    pub message: String,
    pub errors: Vec<ValidationError>,
}

impl IntoResponse for ValidationResponse {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

// Helper function to create a validation error response
pub fn validation_error(message: &str, errors: Vec<ValidationError>) -> ValidationResponse {
    ValidationResponse {
        status: "error".to_string(),
        message: message.to_string(),
        errors,
    }
}

// Helper function to validate string fields
pub fn validate_string_field(
    field_name: &str,
    value: &str,
    min_length: usize,
    max_length: Option<usize>,
) -> Option<ValidationError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Some(ValidationError {
            field: field_name.to_string(),
            message: format!("{} cannot be empty", field_name),
        });
    }

    if trimmed.len() < min_length {
        return Some(ValidationError {
            field: field_name.to_string(),
            message: format!("{} must be at least {} characters", field_name, min_length),
        });
    }

    if let Some(max) = max_length {
        if trimmed.len() > max {
            return Some(ValidationError {
                field: field_name.to_string(),
                message: format!("{} cannot exceed {} characters", field_name, max),
            });
        }
    }

    None
}

// Validate UUID format in string
pub fn validate_uuid(field_name: &str, value: &str) -> Option<ValidationError> {
    match Uuid::parse_str(value) {
        Ok(_) => None,
        Err(_) => Some(ValidationError {
            field: field_name.to_string(),
            message: format!("{} must be a valid UUID", field_name),
        }),
    }
}

// Handle JSON extraction errors with proper response
pub async fn handle_json_extraction<T>(
    json_result: Result<Json<T>, JsonRejection>,
) -> Result<Json<T>, ValidationResponse> {
    json_result.map_err(|err| {
        error!("JSON extraction error: {:?}", err);

        let message = match &err {
            JsonRejection::JsonDataError(_) => "Invalid JSON data",
            JsonRejection::JsonSyntaxError(_) => "Invalid JSON syntax",
            JsonRejection::MissingJsonContentType(_) => {
                "Content-Type header is not application/json"
            }
            JsonRejection::BytesRejection(_) => "Error extracting request body",
            _ => "Error processing JSON payload",
        };

        ValidationResponse {
            status: "error".to_string(),
            message: message.to_string(),
            errors: vec![ValidationError {
                field: "request_body".to_string(),
                message: format!("{}: {}", message, err),
            }],
        }
    })
}
