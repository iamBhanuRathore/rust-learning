mod routes;

use axum::Router;
use tokio::net::TcpListener;
use routes::home_routes;

#[tokio::main]
async fn main() {
    // Combine all routes
    let app = Router::new().merge(home_routes::home_routes());

    // Start the server
    let listener = TcpListener::bind("127.0.0.1:5173").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
