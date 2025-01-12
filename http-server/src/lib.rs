use axum::{
    http::{HeaderValue, Method},
    Router,
};
use tower_http::cors::{
    // Any,
    CorsLayer,
};

// Function to start the HTTP server
pub async fn run(app: Router) {
    // Bind the server to localhost on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    // Serve the Axum app using the listener
    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}

pub fn configure_cors() -> CorsLayer {
    let cors = CorsLayer::new()
        // .allow_methods(Any) // Allow all methods
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PATCH,
            Method::PUT,
        ])
        // .allow_origin(Any) // Allow all origins
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:8080".parse::<HeaderValue>().unwrap(),
        ]);
    cors
}
