mod setup;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use bytes::Bytes;
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
    let response = app.oneshot(request).await.unwrap();

    // Check that the response has a 200 status code
    assert_eq!(response.status(), StatusCode::OK);

    // Check the response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert_eq!(&body[..], b"ok");
}
