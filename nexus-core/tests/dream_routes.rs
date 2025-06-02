mod setup;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
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

    let response = app.clone().oneshot(request).await.unwrap();

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

#[tokio::test]
async fn test_interpret_dream_validation() {
    let app = setup::create_test_server().router();

    // Test with empty dream text
    let request_body = json!({
        "dream_text": ""
    });

    let request = Request::builder()
        .uri("/api/interpret_dream")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    // Should get a bad request response
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    // Validate error response format
    assert_eq!(body.get("status").unwrap().as_str().unwrap(), "error");
    assert!(body.get("errors").is_some(), "Missing 'errors' field");
}

#[tokio::test]
#[ignore] // Requires a running database
async fn test_create_dream_endpoint() {
    let app = setup::create_test_server().router();

    let request_body = json!({
        "text": "I was swimming in a green ocean.",
        "interpretation": "The dream represents your comfort with emotions."
    });

    let request = Request::builder()
        .uri("/api/dreams")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert!(body.get("id").is_some(), "Missing 'id' field");
    assert_eq!(body.get("status").unwrap().as_str().unwrap(), "success");

    // Verify that the ID is a valid UUID
    let id_str = body.get("id").unwrap().as_str().unwrap();
    assert!(Uuid::parse_str(id_str).is_ok(), "ID is not a valid UUID");
}

#[tokio::test]
async fn test_create_dream_validation() {
    let app = setup::create_test_server().router();

    // Test with empty text
    let request_body = json!({
        "text": "",
        "interpretation": "The dream represents your comfort with emotions."
    });

    let request = Request::builder()
        .uri("/api/dreams")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    // Should get a bad request response
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Test with empty interpretation
    let request_body = json!({
        "text": "I was swimming in a green ocean.",
        "interpretation": ""
    });

    let request = Request::builder()
        .uri("/api/dreams")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    // Should get a bad request response
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[ignore] // Requires a running database
async fn test_list_dreams_endpoint() {
    let app = setup::create_test_server().router();

    let request = Request::builder()
        .uri("/api/dreams")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    // The response should be an array
    assert!(body.is_array(), "Response should be an array of dreams");
}

#[tokio::test]
#[ignore] // Requires a running database
async fn test_get_dream_by_id() {
    let app = setup::create_test_server().router();

    // First create a dream
    let create_request_body = json!({
        "text": "I was riding a dragon.",
        "interpretation": "The dream symbolizes power and freedom."
    });

    let create_request = Request::builder()
        .uri("/api/dreams")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::to_vec(&create_request_body).unwrap(),
        ))
        .unwrap();

    let create_response = app.clone().oneshot(create_request).await.unwrap();
    assert_eq!(create_response.status(), StatusCode::OK);

    let create_body_bytes = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let create_body: Value = serde_json::from_slice(&create_body_bytes).unwrap();

    let id = create_body.get("id").unwrap().as_str().unwrap();

    // Now retrieve the dream by ID
    let get_request = Request::builder()
        .uri(format!("/api/dreams/{}", id))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let get_response = app.clone().oneshot(get_request).await.unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);

    let get_body_bytes = axum::body::to_bytes(get_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let get_body: Value = serde_json::from_slice(&get_body_bytes).unwrap();

    assert_eq!(
        get_body.get("id").unwrap().as_str().unwrap(),
        id,
        "ID should match"
    );
    assert_eq!(
        get_body.get("text").unwrap().as_str().unwrap(),
        "I was riding a dragon.",
        "Text should match"
    );
    assert_eq!(
        get_body.get("interpretation").unwrap().as_str().unwrap(),
        "The dream symbolizes power and freedom.",
        "Interpretation should match"
    );
}

#[tokio::test]
#[ignore] // Requires a running database
async fn test_get_nonexistent_dream() {
    let app = setup::create_test_server().router();

    // Generate a random UUID that (almost certainly) doesn't exist
    let random_id = Uuid::new_v4();

    let request = Request::builder()
        .uri(format!("/api/dreams/{}", random_id))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    // Should get a not found response
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(body.get("status").unwrap().as_str().unwrap(), "error");
    assert_eq!(body.get("code").unwrap().as_str().unwrap(), "NOT_FOUND");
}
