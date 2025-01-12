// Import necessary modules and libraries
mod controllers; // Reference to the controllers module
mod middleware;
mod routes; // Reference to the routes module // Reference to the middleware module
use axum::Router; // Axum's Router type
use http_server::run;
use routes::user_routes::users_routes; // Custom function to run the HTTP server
                                       // use routes::home_routes::create_routes; // Function to create the application routes

#[tokio::main] // Marks the main function as asynchronous, using Tokio as the async runtime
async fn main() {
    // Create the app by defining the routes
    // Combine all API routes under `/api/v1`
    let app: Router = Router::new().nest("/api/v1", users_routes());
    // .nest("/api/v1", user_routes())
    // .nest("/api/v1", order_routes());

    // Start the application and bind it to a port
    run(app).await;
}

// Notes:
// - `cargo watch -x run` will rebuild and run the project on file changes (useful during development).
// - `cargo run` will run the project normally.
// - `cargo add serde --features derive,alloc,rc` will add the serde crate with the derive, alloc, and rc features.
