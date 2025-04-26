use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn test_health_endpoint() {
    // Create a simple router
    let app = axum::Router::new().route("/health", axum::routing::get(|| async { "ok" }));

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
