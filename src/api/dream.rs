use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
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

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

// Enum to represent different response types
enum ApiResponse<T> {
    Success(T),
    Error(ErrorResponse),
    NotFound(ErrorResponse),
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiResponse::Success(data) => (StatusCode::OK, Json(data)).into_response(),
            ApiResponse::Error(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response()
            }
            ApiResponse::NotFound(err) => (StatusCode::NOT_FOUND, Json(err)).into_response(),
        }
    }
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

// Handler for interpreting dreams through AI
pub async fn interpret_dream_handler(
    State(state): State<AppState>,
    Json(payload): Json<DreamRequest>,
) -> impl IntoResponse {
    println!("Interpreting dream: {}", payload.dream_text);

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
                        println!("Dream saved with ID: {}", id);
                        Some(id)
                    }
                    Err(e) => {
                        eprintln!("Error saving dream: {:?}", e);
                        None
                    }
                }
            } else {
                None
            };

            (
                StatusCode::OK,
                Json(DreamResponse {
                    interpretation,
                    status: "success".to_string(),
                    id: dream_id,
                }),
            )
        }
        Err(e) => {
            eprintln!("Error interpreting dream: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DreamResponse {
                    interpretation: format!("Could not interpret dream: {:?}", e),
                    status: "error".to_string(),
                    id: None,
                }),
            )
        }
    }
}

// Handler for creating a new dream in the database
pub async fn create_dream_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateDreamRequest>,
) -> impl IntoResponse {
    match Dream::create(&state.db_pool, &payload.text, &payload.interpretation).await {
        Ok(id) => ApiResponse::Success(CreateDreamResponse {
            id,
            status: "success".to_string(),
        }),
        Err(e) => {
            eprintln!("Error creating dream: {:?}", e);
            ApiResponse::Error(ErrorResponse {
                status: "error".to_string(),
                message: format!("Failed to save dream: {:?}", e),
            })
        }
    }
}

// Handler for listing all dreams
pub async fn list_dreams_handler(State(state): State<AppState>) -> impl IntoResponse {
    match Dream::list(&state.db_pool).await {
        Ok(dreams) => ApiResponse::Success(dreams),
        Err(e) => {
            eprintln!("Error listing dreams: {:?}", e);
            ApiResponse::Error(ErrorResponse {
                status: "error".to_string(),
                message: format!("Failed to list dreams: {:?}", e),
            })
        }
    }
}

// Handler for getting a specific dream by ID
pub async fn get_dream_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match Dream::get_by_id(&state.db_pool, id).await {
        Ok(Some(dream)) => ApiResponse::Success(dream),
        Ok(None) => ApiResponse::NotFound(ErrorResponse {
            status: "error".to_string(),
            message: format!("Dream with id {} not found", id),
        }),
        Err(e) => {
            eprintln!("Error getting dream: {:?}", e);
            ApiResponse::Error(ErrorResponse {
                status: "error".to_string(),
                message: format!("Failed to get dream: {:?}", e),
            })
        }
    }
}
