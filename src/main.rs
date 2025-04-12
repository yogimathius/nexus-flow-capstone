use axum_server::create_app;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Setup logging
    tracing_subscriber::fmt::init();

    // Create the app using our library function
    let app = create_app();

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started on port 3000");
    axum::serve(listener, app).await.unwrap();
}
