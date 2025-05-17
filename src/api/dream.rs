use axum::{
    extract::{Json, Path, Query, State, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;

use crate::AppState;
use crate::api::validation::{ValidationResponse, handle_json_extraction, validate_string_field};
use crate::error::AppError;
use crate::models::dream::Dream;

// Request and response types
#[derive(Deserialize)]
pub struct DreamRequest {
    pub dream_text: String,
    #[serde(default)]
    pub save: bool, // Optional parameter to save the dream, defaults to false for backward compatibility
}

#[derive(Serialize)]
pub struct DreamResponse {
    pub interpretation: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_uuid_option")]
    pub id: Option<Uuid>, // Optional ID field present when dream is saved
}

#[derive(Deserialize)]
pub struct CreateDreamRequest {
    pub text: String,
    pub interpretation: String,
}

#[derive(Serialize)]
pub struct CreateDreamResponse {
    pub id: Uuid,
    pub status: String,
}

/// Standard response format for all API endpoints
#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

// Enum to represent different response types
enum ApiResponse<T> {
    Success(T),
    Error(StatusCode, ApiErrorResponse),
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiResponse::Success(data) => (StatusCode::OK, Json(data)).into_response(),
            ApiResponse::Error(status, err) => (status, Json(err)).into_response(),
        }
    }
}

// Helper function to convert AppError to ApiErrorResponse
fn app_error_to_response(err: AppError) -> (StatusCode, ApiErrorResponse) {
    let (status, message, code) = match &err {
        AppError::DatabaseError(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
            Some("DB_ERROR".to_string()),
        ),
        AppError::NotFoundError(e) => (
            StatusCode::NOT_FOUND,
            format!("Not found: {}", e),
            Some("NOT_FOUND".to_string()),
        ),
        AppError::AuthorizationError => (
            StatusCode::UNAUTHORIZED,
            "Authorization error".to_string(),
            Some("AUTH_ERROR".to_string()),
        ),
        AppError::ApiKeyError(e) => (
            StatusCode::UNAUTHORIZED,
            format!("API key error: {}", e),
            Some("API_KEY_ERROR".to_string()),
        ),
        AppError::RateLimitedError => (
            StatusCode::TOO_MANY_REQUESTS,
            "Rate limit exceeded".to_string(),
            Some("RATE_LIMITED".to_string()),
        ),
        AppError::McpError(e) => (
            StatusCode::SERVICE_UNAVAILABLE,
            format!("MCP service error: {}", e),
            Some("MCP_ERROR".to_string()),
        ),
        AppError::ExternalServiceError(e) => (
            StatusCode::BAD_GATEWAY,
            format!("External service error: {}", e),
            Some("EXTERNAL_SERVICE_ERROR".to_string()),
        ),
        AppError::SerializationError(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Serialization error: {}", e),
            Some("SERIALIZATION_ERROR".to_string()),
        ),
        AppError::ApplicationError(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Application error: {}", e),
            Some("APP_ERROR".to_string()),
        ),
        AppError::HttpError(e) => (
            StatusCode::BAD_GATEWAY,
            format!("HTTP error: {}", e),
            Some("HTTP_ERROR".to_string()),
        ),
    };

    (
        status,
        ApiErrorResponse {
            status: "error".to_string(),
            message,
            code,
            details: None,
        },
    )
}

// Helper function to create a not found error response
fn not_found_response(resource: &str, id: &str) -> (StatusCode, ApiErrorResponse) {
    (
        StatusCode::NOT_FOUND,
        ApiErrorResponse {
            status: "error".to_string(),
            message: format!("{} with id {} not found", resource, id),
            code: Some("NOT_FOUND".to_string()),
            details: None,
        },
    )
}

// Helper function to serialize UUID Option as String Option
fn serialize_uuid_option<S>(id: &Option<Uuid>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match id {
        Some(uuid) => serializer.serialize_some(&uuid.to_string()),
        None => serializer.serialize_none(),
    }
}

// Query parameters for pagination and filtering
#[derive(Debug, Deserialize, Default)]
pub struct DreamQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    // Add more query parameters for filtering and sorting later
}

impl DreamQuery {
    // Default page size if not specified
    pub fn get_limit(&self) -> u32 {
        self.limit.unwrap_or(10).min(100) // Maximum of 100 items per page
    }

    // Get the page number (0-based for database offset calculations)
    pub fn get_page(&self) -> u32 {
        self.page.unwrap_or(1).max(1) - 1 // Ensure page is at least 1, then convert to 0-based
    }

    // Calculate the offset for SQL queries
    pub fn get_offset(&self) -> u32 {
        self.get_page() * self.get_limit()
    }
}

// Handler for interpreting dreams through AI
pub async fn interpret_dream_handler(
    State(state): State<AppState>,
    json_result: Result<Json<DreamRequest>, JsonRejection>,
) -> impl IntoResponse {
    // Use the JSON extraction helper to handle JSON errors
    let Json(payload) = match handle_json_extraction(json_result).await {
        Ok(json) => json,
        Err(validation_response) => return validation_response.into_response(),
    };

    info!("Interpreting dream: {}", payload.dream_text);

    // Validate the dream text using the validation helper
    let mut validation_errors = Vec::new();
    if let Some(err) = validate_string_field("dream_text", &payload.dream_text, 1, Some(10000)) {
        validation_errors.push(err);
    }

    if !validation_errors.is_empty() {
        return ValidationResponse {
            status: "error".to_string(),
            message: "Invalid dream text".to_string(),
            errors: validation_errors,
        }
        .into_response();
    }

    match state
        .ai_client
        .request_dream_interpretation(&payload.dream_text)
        .await
    {
        Ok(interpretation) => {
            // If save is true, save the dream to the database
            let dream_id = if payload.save {
                match Dream::create(&state.db_pool, &payload.dream_text, &interpretation).await {
                    Ok(id) => {
                        info!("Dream saved with ID: {}", id);
                        Some(id)
                    }
                    Err(e) => {
                        error!("Error saving dream: {:?}", e);
                        None
                    }
                }
            } else {
                None
            };

            (ApiResponse::Success(DreamResponse {
                interpretation,
                status: "success".to_string(),
                id: dream_id,
            }))
            .into_response()
        }
        Err(e) => {
            error!("Error interpreting dream: {:?}", e);
            let (status, response) = app_error_to_response(e);
            (ApiResponse::<DreamResponse>::Error(status, response)).into_response()
        }
    }
}

// Handler for creating a new dream in the database
pub async fn create_dream_handler(
    State(state): State<AppState>,
    json_result: Result<Json<CreateDreamRequest>, JsonRejection>,
) -> impl IntoResponse {
    // Use the JSON extraction helper to handle JSON errors
    let Json(payload) = match handle_json_extraction(json_result).await {
        Ok(json) => json,
        Err(validation_response) => return validation_response.into_response(),
    };

    // Validate request using validation helpers
    let mut validation_errors = Vec::new();

    if let Some(err) = validate_string_field("text", &payload.text, 1, Some(10000)) {
        validation_errors.push(err);
    }

    if let Some(err) =
        validate_string_field("interpretation", &payload.interpretation, 1, Some(10000))
    {
        validation_errors.push(err);
    }

    if !validation_errors.is_empty() {
        return ValidationResponse {
            status: "error".to_string(),
            message: "Validation failed".to_string(),
            errors: validation_errors,
        }
        .into_response();
    }

    match Dream::create(&state.db_pool, &payload.text, &payload.interpretation).await {
        Ok(id) => (ApiResponse::Success(CreateDreamResponse {
            id,
            status: "success".to_string(),
        }))
        .into_response(),
        Err(e) => {
            error!("Error creating dream: {:?}", e);
            let (status, response) = app_error_to_response(AppError::DatabaseError(e));
            (ApiResponse::<CreateDreamResponse>::Error(status, response)).into_response()
        }
    }
}

// Handler for listing all dreams with pagination, filtering, and sorting
pub async fn list_dreams_handler(
    State(state): State<AppState>,
    Query(params): Query<DreamQuery>,
) -> impl IntoResponse {
    let limit = params.get_limit();
    let offset = params.get_offset();

    // Default sorting by created_at in descending order
    let sort_field = SortField::CreatedAt;
    let sort_order = SortOrder::Descending;

    // Get tag filter from query params if available (to be added later)
    let tag_filter = None;

    // Get search text from query params if available (to be added later)
    let search_text = None;

    match Dream::list_paginated(
        &state.db_pool,
        offset,
        limit,
        sort_field,
        sort_order,
        tag_filter,
        search_text,
    )
    .await
    {
        Ok(paginated_dreams) => (ApiResponse::Success(paginated_dreams)).into_response(),
        Err(e) => {
            error!("Error listing dreams: {:?}", e);
            let (status, response) = app_error_to_response(AppError::DatabaseError(e));
            (ApiResponse::<PaginatedDreams>::Error(status, response)).into_response()
        }
    }
}

// Handler for getting a specific dream by ID
pub async fn get_dream_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match Dream::get_by_id(&state.db_pool, id).await {
        Ok(Some(dream)) => (ApiResponse::Success(dream)).into_response(),
        Ok(None) => {
            let (status, response) = not_found_response("Dream", &id.to_string());
            (ApiResponse::<Dream>::Error(status, response)).into_response()
        }
        Err(e) => {
            error!("Error getting dream: {:?}", e);
            let (status, response) = app_error_to_response(AppError::DatabaseError(e));
            (ApiResponse::<Dream>::Error(status, response)).into_response()
        }
    }
}
