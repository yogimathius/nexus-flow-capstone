mod setup;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use serde_json::{Value, json};
use tower::ServiceExt;

#[tokio::test]
#[ignore]
async fn test_interpret_dream_endpoint() {
    let app = setup::create_test_server().router();

    let dream_text = "I was flying over a city made of crystal.";

    let request_body = json!({
        "dream_text": dream_text
    });

    let request = Request::builder()
        .uri("/api/interpret_dream")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert!(
        body.get("interpretation").is_some(),
        "Missing 'interpretation' field"
    );
    assert!(body.get("status").is_some(), "Missing 'status' field");

    assert_eq!(
        body.get("status").unwrap().as_str().unwrap(),
        "success",
        "Status should be 'success'"
    );
}

// Test the structure for storing dreams - this will be implemented later
#[tokio::test]
#[ignore] // Ignore until database is implemented
async fn test_store_dream() {
    // This test will validate storing dreams in the database
    // Will be implemented after the database models are created
}

// Test retrieving dreams - this will be implemented later
#[tokio::test]
#[ignore] // Ignore until database is implemented
async fn test_get_dreams() {
    // This test will validate retrieving dreams from the database
    // Will be implemented after the database models are created
}
