// Handler function for the root endpoint
pub async fn home_controller() -> &'static str {
    "Hello, World!" // Returns a static response (similar to res.send in Express)
}
