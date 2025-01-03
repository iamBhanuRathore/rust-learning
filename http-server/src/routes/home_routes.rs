use crate::controllers::home_controller;
use axum::{routing::get, Router};

pub fn home_routes() -> Router {
    Router::new().route("/", get(home_controller::home_controller))
}
