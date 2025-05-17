mod setup;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn test_health_endpoint() {
    // Create test server
    let app = setup::create_test_server().router();

    // Create a request to the health endpoint
    let request = Request::builder()
        .uri("/health")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    // Send the request and get the response
    let response = app.clone().oneshot(request).await.unwrap();

    // Check that the response has a 200 status code
    assert_eq!(response.status(), StatusCode::OK);

    // Check the response body for the new standardized format
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    // Verify expected JSON response fields
    assert!(body.get("status").is_some(), "Missing 'status' field");
    assert!(body.get("message").is_some(), "Missing 'message' field");

    assert_eq!(
        body.get("status").unwrap().as_str().unwrap(),
        "success",
        "Status should be 'success'"
    );

    assert_eq!(
        body.get("message").unwrap().as_str().unwrap(),
        "Service is running normally",
        "Message should be 'Service is running normally'"
    );
}
