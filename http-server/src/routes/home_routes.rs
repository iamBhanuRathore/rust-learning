use axum::{routing::get, Router}; // Import Axum's Router and routing functionality (like Express route handlers)

use crate::controllers::home_controller::home_controller; // Import the home controller function from the controllers module

// Function to define and create application routes
pub fn create_routes() -> Router {
    // Create a new router and attach the route for the root ("/") endpoint
    Router::new().route("/other", get(home_controller)) // Similar to Express: app.get('/', controller)
}

// pub async fn mirror_body_string() -> Router {}
