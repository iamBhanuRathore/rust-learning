use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

#[derive(Clone)]
pub struct User {
    pub id: String,
}

pub async fn auth_middleware(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok());
    let user_id = match token {
        Some(token) => validate_token(token).await,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    match user_id {
        Some(id) => {
            req.extensions_mut().insert(User { id });
            Ok(next.run(req).await)
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

// Simulated token validation function
async fn validate_token(token: &str) -> Option<String> {
    // Replace with actual validation logic, e.g., parsing a JWT
    if token == "valid-token" {
        Some("user123".to_string()) // Example user ID
    } else {
        None
    }
}
